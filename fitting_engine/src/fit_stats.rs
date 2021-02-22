use crate::stats::capacitor::Capacitor;
use crate::stats::defense::Defense;
use crate::stats::drone::Drone;
use crate::stats::fitting::Fitting;
use crate::stats::movement::Movement;
use crate::stats::sensor::Sensor;

#[derive(Debug, Clone)]
pub struct FitStats {}

impl FitStats {
    pub fn new(
        fitting: Fitting,
        capacitor: Capacitor,
        defense: Defense,
        movement: Movement,
        sensor: Sensor,
        drone: Drone,
    ) -> Self {
        Self {}
    }
}
