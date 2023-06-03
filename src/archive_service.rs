use std::time::SystemTime;

use crate::archive::{PowerUnit, EnergyUnit, Archive};
use crate::options::RunOptions;
use crate::shelly_api;
use crate::shelly_plug_s_meter::ShellyPlugSMeter;


pub fn archive_service(
    connection: rusqlite::Connection,
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
                if storage_size != 0 {
                    if let Ok(db_size) = get_db_size(&connection) {
                        if db_size > (storage_size * 1024) {
                            remove_old_entries(&connection).unwrap();
                        }
                    }
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}

/// Gets the data from the shelly api endpoint /meter/0, converts the timestamp to UTC
/// and calls [`create_entry`] to save the data in the database
fn archive_data(
    connection: &rusqlite::Connection,
    runoptions: &RunOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let json = shelly_api::get_meter_status_from_shelly_plug_s(runoptions)?;
    let mut meter = serde_json::from_str::<ShellyPlugSMeter>(&json)?;
    let utc_offset = shelly_api::get_utc_offset_from_shelly_plug_s(runoptions)?;
    meter.timestamp = (meter.timestamp as i128 - utc_offset as i128) as u64;
    create_entry(connection, &meter)
}

/// Insert an entry in the archive table.
/// Power in milliwatts
/// Energy in milliwatts minutes (measured at the last round minute)
/// Timestamp is the moment of measuring
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
                    meter.total as u64 * 1000,
                ),
            )?;
        }
    }
    Ok(())
}

/// Gets sqlite database size with the pragmas page_count, freelist_count and page_size
/// # Calculation
/// (page_count - freelist_count) * page_size
/// # Returns
/// Database size in bytes
/// # Caution
/// This is not the file size of the database, because the empty freelist pages occupies space. So the file size is probably bigger.
fn get_db_size(connection: &rusqlite::Connection) -> Result<usize, Box<dyn std::error::Error>> {
    let size = connection.query_row(GET_DB_SIZE, [], |row| row.get::<usize, usize>(0))?;
    Ok(size)
}
/// Deletes the oldest 5 entries.
fn remove_old_entries(connection: &rusqlite::Connection) -> Result<(), Box<dyn std::error::Error>> {
    connection.execute(DELETE_ENTRIES, [1])?;
    Ok(())
}

/// Creates DB if not existent.
/// # Parameter
/// archive_path should be None, if InMemory should be used
pub fn init_archive(memory: bool) -> Result<rusqlite::Connection, Box<dyn std::error::Error>> {
    let connection = get_connection(memory)?;
    let mut statement = connection.prepare(CHECK_TABLE)?;
    let mut rows = statement.query(&[(":name", "Archive")])?;
    let row = rows.next()?;
    if let Some(r) = row {
        let count = r.get::<usize, u8>(0)?;
        if count == 0 {
            connection.execute_batch(CREATE_TABLES)?;
        }
    } else {
        connection.execute_batch(CREATE_TABLES)?;
    }
    drop(rows);
    drop(statement);
    Ok(connection)
}

fn get_connection(memory: bool) -> Result<rusqlite::Connection, Box<dyn std::error::Error>> {
    return if memory {
        Ok(rusqlite::Connection::open(":memory:")?)
    } else {
        return if std::path::Path::new(ARCHIVE_PATH).exists() {
            Ok(rusqlite::Connection::open(ARCHIVE_PATH)?)
        } else {
            Ok(rusqlite::Connection::open(ARCHIVE_PATH)?)
        };
    };
}

pub fn get_entries(
    memory: bool,
    from: u32,
    to: u32,
) -> Result<Vec<Archive>, Box<dyn std::error::Error>> {
    let connection = get_connection(memory)?;
    let mut statement = connection.prepare(GET_ENTRIES)?;
    let rows = statement.query_map((1, from, to), |row| {
        Ok(Archive {
            timestamp: row.get(0)?,
            plug_id: row.get(1)?,
            power: row.get(2)?,
            power_unit: row.get(3)?,
            energy: row.get(4)?,
            energy_unit: row.get(5)?,
            total_energy: row.get(6)?,
        })
    })?;
    let mut vec = Vec::new();
    for row in rows {
        vec.push(row?);
    }
    match vec.len() > 1000 {
        true => Ok(aggregate_archive_data(vec)),
        false => Ok(vec),
    }
}

fn aggregate_archive_data(vec: Vec<Archive>) -> Vec<Archive> {
    let diff = vec[1].timestamp - vec[0].timestamp;
    match diff {
        0..=59 => aggregate_to_minute(vec),
        60..=3599 => aggregate_to_hour(vec),
        3600..=86399 => aggregate_to_day(vec),
        _ => vec,
    }
}

fn aggregate_to_day(vec: Vec<Archive>) -> Vec<Archive> {
    todo!()
}

fn aggregate_to_hour(vec: Vec<Archive>) -> Vec<Archive> {
    todo!()
}

fn aggregate_to_minute(vec: Vec<Archive>) -> Vec<Archive> {
    todo!()
}

pub const ARCHIVE_PATH: &str = "./archive.db";
const CHECK_TABLE: &str = "SELECT Count(*) FROM sqlite_master WHERE type='table' AND name= :name ;";
const CREATE_TABLES: &str = " BEGIN;
 CREATE TABLE Archive(timestamp INTEGER, plug_id INTEGER, power INTEGER,power_unit INTEGER, energy INTEGER, energy_unit INTEGER, total_energy INTEGER, PRIMARY KEY(timestamp, plug_id)) WITHOUT ROWID;
 CREATE TABLE Archive_autoinc(num INTEGER); 
 INSERT INTO Archive_autoinc(num) VALUES(0);
 CREATE TRIGGER insert_trigger BEFORE INSERT ON Archive BEGIN UPDATE Archive_autoinc SET num=num+1;END;
 CREATE TABLE ShellyPlugs(id INTEGER PRIMARY KEY AUTOINCREMENT, label TEXT, room TEXT, product_name TEXT, ip TEXT);
 INSERT INTO ShellyPlugs(label,product_name) VALUES ('default','Shelly Plug S');
 COMMIT;
 ";
const CHECK_ENTRY: &str = "SELECT Count(*) from Archive WHERE plug_id = ?1 AND timestamp = ?2;";
const GET_ENTRIES: &str = "SELECT timestamp,plug_id,power,power_unit,energy,energy_unit,total_energy from Archive WHERE plug_id = ?1 AND timestamp >= ?2 AND timestamp <= ?3;";
const ADD_ENTRY: &str = "INSERT INTO Archive(timestamp,plug_id,power, power_unit, energy, energy_unit, total_energy) Values(?1,1,?2,?3,?4,?5,?6);";
const DELETE_ENTRIES: &str = "DELETE FROM Archive WHERE plug_id = ?1 AND timestamp IN (SELECT timestamp FROM Archive ORDER BY timestamp ASC LIMIT 5);";
const GET_DB_SIZE: &str = "Select (((Select * From PRAGMA_PAGE_COUNT) - (Select * From PRAGMA_FREELIST_COUNT)) * (Select * From PRAGMA_PAGE_SIZE));";

#[cfg(test)]
mod tests {
    use crate::shelly_plug_s_meter::ShellyPlugSMeter;

    use super::*;

    #[test]
    fn init_archive_test() {
        let connection = init_archive(true).unwrap();
        let tables = ["Archive", "Archive_autoinc", "ShellyPlugs"];
        for table in tables {
            let mut statement = connection.prepare(CHECK_TABLE).unwrap();
            let mut rows = statement.query(&[(":name", table)]).unwrap();
            let row = rows.next().unwrap();
            assert!(row.is_some());
            if let Some(row) = row {
                let count = row.get::<usize, u8>(0).unwrap();
                assert_eq!(count, 1, "expect={} rowcount= {}", 1, count)
            }
        }
    }

    #[test]
    fn create_entry_test() {
        let connection = init_archive(true).unwrap();
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
            assert_eq!(count, 1, "expect={} rowcount= {}", 1, count)
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
            assert_eq!(count, 10, "expect={} rowcount= {}", 10, count)
        }
    }

    #[test]
    fn remove_old_entries_test() {
        let connection = init_archive(true).unwrap();
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
            assert_eq!(count, 5, "expect={} rowcount= {}", 5, count)
        }
    }
}
