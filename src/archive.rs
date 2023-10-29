use rusqlite;

use crate::aggreated_archive_data::{EnergyData, GetEnergyData};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Archive {
    pub timestamp: u64,
    pub plug_id: u8,
    pub power: u32,
    pub power_unit: PowerUnit,
    pub energy: u32,
    pub energy_unit: EnergyUnit,
    pub total_energy: u64,
}

impl GetEnergyData for Archive {
    fn get_energy_data(&self) -> EnergyData {
        EnergyData {
            timestamp: self.timestamp,
            plug_id: self.plug_id,
            energy: self.energy,
            energy_unit: self.energy_unit,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub enum PowerUnit {
    Milliwatt,
    Watt,
    Kilowatt,
}

impl From<i64> for PowerUnit {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Milliwatt,
            1 => Self::Watt,
            2 => Self::Kilowatt,
            _ => {
                let name = nameof::name_of_type!(Self);
                panic!("{}", format!("Could not parse value to {name}"));
            }
        }
    }
}

impl rusqlite::types::FromSql for PowerUnit {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Null => Ok(PowerUnit::Milliwatt),
            rusqlite::types::ValueRef::Integer(int) => Ok(int.into()),
            rusqlite::types::ValueRef::Real(_)
            | rusqlite::types::ValueRef::Text(_)
            | rusqlite::types::ValueRef::Blob(_) => {
                let name = nameof::name_of_type!(Self);
                panic!("{}", format!("Could not parse Real to {name}"));
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub enum EnergyUnit {
    MilliwattSeconds,
    MiliwattMinute,
    WattSeconds,
    WattMinutes,
    WattHours,
    KilowattHours,
}

impl From<i64> for EnergyUnit {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::MilliwattSeconds,
            1 => Self::MiliwattMinute,
            2 => Self::WattSeconds,
            3 => Self::WattMinutes,
            4 => Self::WattHours,
            5 => Self::KilowattHours,
            _ => {
                let name = nameof::name_of_type!(Self);
                panic!("{}", format!("Could not parse value to {name}"));
            }
        }
    }
}

impl rusqlite::types::FromSql for EnergyUnit {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Null => Ok(EnergyUnit::WattMinutes),
            rusqlite::types::ValueRef::Integer(int) => Ok(int.into()),
            rusqlite::types::ValueRef::Real(_)
            | rusqlite::types::ValueRef::Text(_)
            | rusqlite::types::ValueRef::Blob(_) => {
                let name = nameof::name_of_type!(Self);
                panic!("{}", format!("Could not parse Real to {name}"));
            }
        }
    }
}
