use serde::{Deserialize, Serialize};

/// Represents a customer
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct State {
    pub steps: i32,
    pub time: f64,
}
