use serde::{Deserialize, Serialize};


/// Represents a customer
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct State {
    pub running: bool,
    pub mode: i32,  // 1 default, 2 asap mode
    pub ml: f64,
    pub progress: i32,
    pub time: f64,
    pub steps: i32,
    pub steps_per_ml: i32,
    pub syringe_size: i32,
}
