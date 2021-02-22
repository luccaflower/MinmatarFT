#[cfg(test)]
mod tests {
    mod any_fit {
        use crate::faction::Faction;
        use crate::fit::Fit;
        use crate::ship::{RigSize, SensorStrengthType, Ship};
        use crate::ship_type::cruiser::CruiserType;
        use crate::ship_type::ShipType;
        use crate::stats::capacitor::Capacitor;
        use crate::stats::defense::Defense;
        use crate::stats::drone::Drone;
        use crate::stats::fitting::Fitting;
        use crate::stats::movement::Movement;
        use crate::stats::sensor::Sensor;
        use once_cell::sync::Lazy;

        pub static SHIP: Lazy<Ship> = Lazy::new(|| {
            Ship::new(
                "Caracal",
                ShipType::Cruiser(CruiserType::T1),
                Faction::Caldari,
                5,
                6,
                4,
                0,
                5,
                3,
                RigSize::Medium,
                SensorStrengthType::Magnetometric,
                Fitting::new(275.0, 275.0, 400, 375.0),
                Defense::new(
                    800, 0.0, 0.0, 0.0, 0.0, 600, 50.0, 40.0, 25.0, 0.0, 1200, 0.0, 25.0, 40.0,
                    50.0, 1200,
                ),
                Movement::new(200.0, 1.25, 1200000, 4.0),
                Sensor::new(32.0, 200, 32.0, 5),
                Drone::new(20000, 25, 25, 0),
                Capacitor::new(400.0, 16, 0.0),
            )
        });

        #[test]
        fn has_a_slot_layout_matching_its_associated_ship() {
            let fit = Fit::new(&SHIP);
            assert_eq!(fit.high_slots.len(), SHIP.high_slots as usize);
            assert_eq!(fit.med_slots.len(), SHIP.med_slots as usize);
            assert_eq!(fit.low_slots.len(), SHIP.low_slots as usize);
        }
    }
}
