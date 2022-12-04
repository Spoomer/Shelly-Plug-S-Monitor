use std::time::SystemTime;

use crate::options::RunOptions;
use crate::shelly_api;
use crate::shelly_plug_s_meter::ShellyPlugSMeter;
use rusqlite;

struct _Archive {
    timestamp: u32,
    plug_id: u8,
    power: u32,
    power_unit: PowerUnit,
    energy: u32,
    energy_unit: EnergyUnit,
    total_energy: u32,
}

enum PowerUnit {
    Milliwatt,
    _Watt,
    _Kilowatt,
}

enum EnergyUnit {
    _MilliwattSeconds,
    MiliwattMinute,
    _WattSeconds,
    _WattMinutes,
    _WattHours,
    _KilowattHours,
}

pub fn archive_data(
    connection: rusqlite::Connection,
    archive_path: &str,
    storage_size: usize,
    runoptions: &RunOptions,
    cancel: &bool,
) {
    let mut last: SystemTime = std::time::UNIX_EPOCH;
    while !cancel {
        if let Ok(elapsed) = last.elapsed() {
            if elapsed.as_secs() >= 60 {
                match make_entry(&connection, runoptions) {
                    Ok(()) => last = SystemTime::now(),
                    Err(_) => (),
                }
            }
        }
        if storage_size != 0 {
            if let Ok(metadata) = std::fs::metadata(archive_path) {
                if metadata.len() > storage_size as u64 {
                    remove_old_entries(&connection);
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}

fn remove_old_entries(connection: &rusqlite::Connection) {
    _ = connection.execute("DELETE FROM Archive WHERE plug_id = ?1 AND timestamp IN (SELECT Top(5) id FROM mytable ORDER BY timestamp ASC) ", [1]);
}

fn make_entry(
    connection: &rusqlite::Connection,
    runoptions: &RunOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = shelly_api::get_from_shelly_plug_s(runoptions)?;
    let meter = serde_json::from_str::<ShellyPlugSMeter>(&json)?;
    //check for entry
    let mut statement = connection
        .prepare("SELECT Count(*) from Archive WHERE plug_id = ?1 AND timestamp = ?2;")?;
    let mut rows = statement.query((1, &meter.timestamp))?;
    if let Some(row) = rows.next()? {
        let count = row.get::<usize, u8>(0)?;
        if count == 0 {
            //write entry
            connection.execute("INSERT INTO Archive(timestamp,plug_id,power, power_unit, energy, energy_unit, total_energy) Values(?1,1,?2,?3,?4,?5,?6)",
            (meter.timestamp, (meter.power * 1000.0) as u32, PowerUnit::Milliwatt as u8, (meter.counters[0] * 1000.0) as u32, EnergyUnit::MiliwattMinute as u8, meter.total * 1000))?;
        }
    }
    Ok(())
}

pub fn init_archive(archive_path: Option<&str>) -> rusqlite::Connection {
    let connection: rusqlite::Connection;
    if let Some(path) = archive_path {
        if std::path::Path::new(path).exists() {
            return rusqlite::Connection::open(path).unwrap();
        }
        connection = rusqlite::Connection::open(path).unwrap();
    } else {
        connection = rusqlite::Connection::open(":memory:").unwrap();
    }

    connection
        .execute("CREATE TABLE Archive_autoinc(num INTEGER);", ())
        .expect("Archive_autoinc init failed");
    connection
        .execute("INSERT INTO Archive_autoinc(num) VALUES(0);", ())
        .expect("Archive_autoinc insert failed");
    connection.execute("CREATE TABLE Archive(timestamp INTEGER, plug_id INTEGER, power INTEGER,power_unit INTEGER, energy INTEGER, energy_unit INTEGER, total_energy INTEGER, PRIMARY KEY(timestamp, plug_id)) WITHOUT ROWID;",())
    .expect("Archive init failed");
    connection.execute("CREATE TRIGGER insert_trigger BEFORE INSERT ON Archive BEGIN UPDATE Archive_autoinc SET num=num+1;END;", ())
    .expect("trigger init failed");

    connection.execute("CREATE TABLE ShellyPlugs(id INTEGER PRIMARY KEY AUTOINCREMENT, label TEXT, room TEXT, product_name TEXT, ip TEXT);",())
    .expect("ShellyPlugs init failed");
    connection
        .execute(
            "INSERT INTO ShellyPlugs(label,product_name) VALUES ('default','Shelly Plug S');",
            (),
        )
        .expect("ShellyPlugs insert failed");

    connection
}
