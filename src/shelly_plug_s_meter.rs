use serde::Deserialize;
#[derive(Deserialize)]
pub struct ShellyPlugSMeter {
    pub power: f32,
    pub overpower: f32,
    pub is_valid: bool,
    pub timestamp: u64,
    pub counters: [f32; 3],
    pub total: u32,
}
