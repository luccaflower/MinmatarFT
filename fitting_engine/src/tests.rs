#[cfg(test)]
mod tests {
    mod any_fit {
        use crate::faction::Faction;
        use crate::fit::Fit;
        use crate::ship::{RigSize, SensorStrengthType, Ship};
        use crate::ship_type::cruiser::CruiserType;
        use crate::ship_type::ShipType;
        use crate::static_module::{ModuleSlot, StaticModule};
        use crate::stats::capacitor::Capacitor;
        use crate::stats::defense::Defense;
        use crate::stats::drone::Drone;
        use crate::stats::fitting::Fitting;
        use crate::stats::movement::*;
        use crate::stats::sensor::Sensor;
        use crate::stats::ModificationType;
        use once_cell::sync::Lazy;
        use std::borrow::Cow;

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

        pub static MODULE_A: Lazy<StaticModule> = Lazy::new(|| {
            StaticModule::new(
                "5MN Microwarpdrive",
                None,
                None,
                None,
                None,
                Some(MovementModifications::new(
                    ModificationType::Additive(2.1),
                    ModificationType::Additive(1.0),
                    ModificationType::Additive(1.0),
                    ModificationType::Additive(1.0),
                )),
                None,
                None,
                None,
                None,
                ModuleSlot::Med,
                None,
            )
        });

        #[test]
        fn has_a_slot_layout_matching_its_associated_ship() {
            let fit = Fit::new(Cow::Borrowed(""), &SHIP);
            assert_eq!(fit.high_slots.len(), SHIP.high_slots as usize);
            assert_eq!(fit.med_slots.len(), SHIP.med_slots as usize);
            assert_eq!(fit.low_slots.len(), SHIP.low_slots as usize);
        }
    }

    mod fit_compression {
        use crate::fit::{CompressedFit, Fit};
        use crate::tests::tests::any_fit::{MODULE_A, SHIP};
        use std::borrow::Cow;
        use std::collections::HashMap;
        use std::ops::Deref;

        #[test]
        fn compresses_into_names_only() {
            let mut ship = Fit::new(Cow::Owned("aaa".to_string()), SHIP.deref());
            ship.add_module(MODULE_A.deref());
            let CompressedFit {
                name,
                ship,
                high_slots,
                mut med_slots,
                low_slots,
            } = ship.compress();
            assert_eq!("aaa", name.to_string());
            assert_eq!("Caracal", ship.to_string());
            assert_eq!("5MN Microwarpdrive", med_slots.pop().unwrap());
            assert!(high_slots.is_empty());
            assert!(med_slots.is_empty());
            assert!(low_slots.is_empty());
        }

        #[test]
        fn decompresses_from_names_only() {
            let high_slots: Vec<String> = Vec::new();
            let med_slots = vec!["5MN Microwarpdrive"];
            let low_slots: Vec<String> = Vec::new();
            let compressed_fit =
                CompressedFit::new("", "Caracal", high_slots, med_slots, low_slots);
            let mut ships = HashMap::new();
            ships.insert(SHIP.name.deref(), SHIP.deref().clone());
            let mut modules = HashMap::new();
            modules.insert(MODULE_A.name.deref(), MODULE_A.deref().clone());
            let fit = compressed_fit.decompress(&ships, &modules).unwrap();
            assert!(fit.ship.clone().eq(SHIP.deref()));
            assert!(fit.med_slots[0]
                .as_ref()
                .unwrap()
                .inner_module
                .clone()
                .eq(MODULE_A.deref()))
        }
    }
}
