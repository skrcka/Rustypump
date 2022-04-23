use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct State {
    pub running: bool,
    pub mode: i32,  // 1 ml/time mode, 2 asap mode, 3 rate mode
    pub pull: bool,
    pub ml: f64,
    pub ml_in_pump: f64,
    pub total_ml: f64,
    pub progress: i32,
    pub time_rate: f64,
    pub steps: i32,
    pub steps_per_ml: i32,
    pub syringe_size: f64,
    pub ip: String,
    pub pause: bool,
    pub bolus_dose: f64,
    pub active_bolus_dose: i32,
    pub bolus_cooldown: f64,
    pub active_bolus_cooldown: f64,
}
