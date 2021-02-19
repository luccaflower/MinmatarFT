use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fitting {
    cpu: f64,
    pg: f64,
    calibration: u8,
}