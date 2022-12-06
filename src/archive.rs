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

pub fn archive_service(
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
                if let Ok(()) = archive_data(&connection, runoptions) {
                    last = SystemTime::now()
                }
            }
        }
        if storage_size != 0 {
            if let Ok(metadata) = std::fs::metadata(archive_path) {
                if metadata.len() > storage_size as u64 {
                    _ = remove_old_entries(&connection);
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}

fn archive_data(
    connection: &rusqlite::Connection,
    runoptions: &RunOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = shelly_api::get_meter_status_from_shelly_plug_s(runoptions)?;
    let meter = serde_json::from_str::<ShellyPlugSMeter>(&json)?;
    create_entry(connection, &meter)
}

fn create_entry(
    connection: &rusqlite::Connection,
    meter: &ShellyPlugSMeter,
) -> Result<(), Box<dyn std::error::Error>> {
    //check for entry
    let mut statement = connection.prepare(CHECK_ENTRY)?;
    let mut rows = statement.query((1, &meter.timestamp))?;
    if let Some(row) = rows.next()? {
        let count = row.get::<usize, u8>(0)?;
        if count == 0 {
            //write entry
            connection.execute(
                ADD_ENTRY,
                (
                    meter.timestamp,
                    (meter.power * 1000.0) as u32,
                    PowerUnit::Milliwatt as u8,
                    (meter.counters[0] * 1000.0) as u32,
                    EnergyUnit::MiliwattMinute as u8,
                    meter.total * 1000,
                ),
            )?;
        }
    }
    Ok(())
}

fn remove_old_entries(connection: &rusqlite::Connection) -> Result<(), Box<dyn std::error::Error>> {
    _ = connection.execute(DELETE_ENTRIES, [1])?;
    Ok(())
}

/// Creates DB if not existent.
/// # Parameter
/// archive_path should be None, if InMemory should be used
pub fn init_archive(
    archive_path: Option<&str>,
) -> Result<rusqlite::Connection, Box<dyn std::error::Error>> {
    let connection: rusqlite::Connection;
    if let Some(path) = archive_path {
        if std::path::Path::new(path).exists() {
            return Ok(rusqlite::Connection::open(path)?);
        }
        connection = rusqlite::Connection::open(path)?;
    } else {
        connection = rusqlite::Connection::open(":memory:")?;
    }

    connection.execute_batch(CREATE_TABLES)?;

    Ok(connection)
}
const CREATE_TABLES : &str = " BEGIN;
 CREATE TABLE Archive(timestamp INTEGER, plug_id INTEGER, power INTEGER,power_unit INTEGER, energy INTEGER, energy_unit INTEGER, total_energy INTEGER, PRIMARY KEY(timestamp, plug_id)) WITHOUT ROWID;
 CREATE TABLE Archive_autoinc(num INTEGER); 
 INSERT INTO Archive_autoinc(num) VALUES(0);
 CREATE TRIGGER insert_trigger BEFORE INSERT ON Archive BEGIN UPDATE Archive_autoinc SET num=num+1;END;
 CREATE TABLE ShellyPlugs(id INTEGER PRIMARY KEY AUTOINCREMENT, label TEXT, room TEXT, product_name TEXT, ip TEXT);
 INSERT INTO ShellyPlugs(label,product_name) VALUES ('default','Shelly Plug S');
 COMMIT;
 ";
const CHECK_ENTRY: &str = "SELECT Count(*) from Archive WHERE plug_id = ?1 AND timestamp = ?2;";
const ADD_ENTRY : &str = "INSERT INTO Archive(timestamp,plug_id,power, power_unit, energy, energy_unit, total_energy) Values(?1,1,?2,?3,?4,?5,?6);";
const DELETE_ENTRIES : &str = "DELETE FROM Archive WHERE plug_id = ?1 AND timestamp IN (SELECT timestamp FROM Archive ORDER BY timestamp ASC LIMIT 5);";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_archive_test() {
        let connection = init_archive(None).unwrap();
        let tables = ["Archive", "Archive_autoinc", "ShellyPlugs"];
        for table in tables {
            let mut statement = connection
                .prepare("SELECT Count(*) FROM sqlite_master WHERE type='table' AND name= :name ;")
                .unwrap();
            let mut rows = statement.query(&[(":name", table)]).unwrap();
            let row = rows.next().unwrap();
            assert!(row.is_some());
            if let Some(row) = row {
                let count = row.get::<usize, u8>(0).unwrap();
                assert!(count == 1, "expect={} rowcount= {}", 1, count)
            }
        }
    }
    #[test]
    fn create_entry_test() {
        let connection = init_archive(None).unwrap();
        let meter = ShellyPlugSMeter {
            power: 1.0,
            overpower: 1.0,
            is_valid: true,
            timestamp: 1,
            counters: [1.0, 2.0, 3.0],
            total: 11,
        };
        // 1
        for _ in 0..10 {
            assert!(create_entry(&connection, &meter).is_ok());
        }
        let mut statement = connection.prepare("SELECT Count(*) FROM Archive;").unwrap();
        let mut rows = statement.query([]).unwrap();
        let row = rows.next().unwrap();
        assert!(row.is_some());
        if let Some(row) = row {
            let count = row.get::<usize, u8>(0).unwrap();
            assert!(count == 1, "expect={} rowcount= {}", 1, count)
        }
        // 10
        for i in 0..10 {
            let meter = ShellyPlugSMeter {
                power: 1.0,
                overpower: 1.0,
                is_valid: true,
                timestamp: i,
                counters: [1.0, 2.0, 3.0],
                total: 11,
            };
            assert!(create_entry(&connection, &meter).is_ok());
        }
        let mut statement = connection.prepare("SELECT Count(*) FROM Archive;").unwrap();
        let mut rows = statement.query([]).unwrap();
        let row = rows.next().unwrap();
        assert!(row.is_some());
        if let Some(row) = row {
            let count = row.get::<usize, u8>(0).unwrap();
            assert!(count == 10, "expect={} rowcount= {}", 10, count)
        }
    }
    #[test]
    fn remove_old_entries_test() {
        let connection = init_archive(None).unwrap();
        for i in 0..10 {
            let meter = ShellyPlugSMeter {
                power: 1.0,
                overpower: 1.0,
                is_valid: true,
                timestamp: i,
                counters: [1.0, 2.0, 3.0],
                total: 11,
            };
            assert!(create_entry(&connection, &meter).is_ok());
        }
        remove_old_entries(&connection).unwrap();
        let mut statement = connection.prepare("SELECT Count(*) FROM Archive;").unwrap();
        let mut rows = statement.query([]).unwrap();
        let row = rows.next().unwrap();
        assert!(row.is_some());
        if let Some(row) = row {
            let count = row.get::<usize, u8>(0).unwrap();
            assert!(count == 5, "expect={} rowcount= {}", 5, count)
        }
    }
}
