use serde::{Deserialize, Serialize};

pub const STEPS_PER_ML: i32 = 500;

/// Represents a customer
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct State {
    pub mode: i32,  // 0 disabled, 1 default
    pub ml: f64,
    pub progress: i32,
    pub time: f64,
    pub steps: i32,
}
