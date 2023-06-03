use crate::archive::EnergyUnit;

pub struct AggregatedArchiveData {
    pub timestamp: u64,
    pub plug_id: u8,
    pub energy: u32,
    pub energy_unit: EnergyUnit,
    pub granularity: Granularity,
}
impl GetEnergyData for AggregatedArchiveData {
    fn get_energy_data(&self) -> EnergyData {
        return EnergyData {
            timestamp: self.timestamp,
            plug_id: self.plug_id,
            energy: self.energy,
            energy_unit: self.energy_unit,
        };
    }
}
pub enum Granularity {
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}
#[derive(serde::Serialize)]
pub struct EnergyData {
    pub timestamp: u64,
    pub plug_id: u8,
    pub energy: u32,
    pub energy_unit: EnergyUnit,
}
pub trait GetEnergyData {
    fn get_energy_data(&self) -> EnergyData;
}
