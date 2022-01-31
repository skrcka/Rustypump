use serde::{Deserialize, Serialize};

pub const stepsPerMl: i32 = 20;

/// Represents a customer
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct State {
    pub enabled: bool,
    pub ml: f64,
    pub progress: i32,
    pub time: f64,
    pub steps: i32,
}
