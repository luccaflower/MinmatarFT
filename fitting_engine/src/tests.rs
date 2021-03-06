use crate::fit::Fit;
use crate::ship::{RigSize, SensorStrengthType, Ship};
use crate::ship_type::cruiser::CruiserType;
use crate::ship_type::ShipType;
use crate::stats::capacitor::Capacitor;
use crate::stats::defense::Defense;
use crate::stats::drone::Drone;
use crate::stats::fitting::Fitting;
use crate::stats::movement::*;
use crate::stats::sensor::Sensor;
use crate::{
    faction::Faction,
    static_module::{ModuleSlot, StaticModule},
    stats::ModificationType,
};
use once_cell::sync::Lazy;
use shoulda::Shoulda;
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
            800.0, 0.0, 0.0, 0.0, 0.0, 600.0, 50.0, 40.0, 25.0, 0.0, 1200.0,
            0.0, 25.0, 40.0, 50.0, 1200.0,
        ),
        Movement::new(200.0, 1.25, 1200000.0, 4.0),
        Sensor::new(32.0, 200.0, 32.0, 5),
        Drone::new(20000, 25, 25, 0),
        Capacitor::new(400.0, 16.0, 0.0),
    )
});

pub static MICROWARPDRIVE: Lazy<StaticModule> = Lazy::new(|| {
    StaticModule::new(
        "5MN Microwarpdrive",
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        MovementModifications::new(
            ModificationType::Additive(2.1),
            ModificationType::Additive(1.0),
            ModificationType::Additive(1.0),
            ModificationType::Additive(1.0),
        ),
        Default::default(),
        Default::default(),
        Default::default(),
        Default::default(),
        ModuleSlot::Med,
        Default::default(),
        false,
    )
});

mod any_fit {
    use super::*;

    #[test]
    fn has_a_slot_layout_matching_its_associated_ship() {
        let fit = Fit::new(Cow::Borrowed(""), &SHIP);
        fit.high_slots.len().should().eq(SHIP.high_slots as usize);
        fit.med_slots.len().should().eq(SHIP.med_slots as usize);
        fit.low_slots.len().should().eq(SHIP.low_slots as usize);
    }

    #[test]
    fn can_add_a_new_module_to_an_empty_slot() {
        let mut fit = Fit::new(Cow::Borrowed(""), &SHIP);
        fit.add_module(&MICROWARPDRIVE);
        fit.med_slots.to_vec().should().contains(|module| {
            module
                .as_ref()
                .filter(|x| x.inner_module.name == MICROWARPDRIVE.name)
                .is_some()
        });
    }
}

mod an_empty_fit {
    use super::*;
    #[test]
    fn matches_the_base_stats_of_its_ship() {
        let fit = Fit::new(Cow::Borrowed(""), &SHIP);
        fit.calculate_stats()
            .fitting
            .should()
            .eq(&SHIP.fitting_stats);
    }
}

mod a_non_empty_fit {
    use super::*;

    #[test]
    fn can_remove_a_module() {
        let mut fit = Fit::new(Cow::Borrowed(""), &SHIP);
        fit.add_module(&MICROWARPDRIVE);
        fit.remove_module(ModuleSlot::Med, 0);
        fit.med_slots
            .to_vec()
            .should()
            .not()
            .contains(|x| x.is_some());
    }
}

mod fit_compression {
    use std::{collections::HashMap, ops::Deref};

    use super::*;
    use crate::fit::CompressedFit;
    use shoulda::Shoulda;

    #[test]
    fn compresses_into_names_only() {
        let mut ship = Fit::new(Cow::Owned("aaa".to_string()), SHIP.deref());
        ship.add_module(MICROWARPDRIVE.deref());
        let compressed = ship.compress();
        compressed.should().eq(CompressedFit::new(
            "aaa",
            "Caracal",
            vec!["5MN Microwarpdrive"],
            Vec::<String>::new(),
            Vec::<String>::new(),
        ));
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
        modules.insert(
            MICROWARPDRIVE.name.deref(),
            MICROWARPDRIVE.deref().clone(),
        );
        let fit = compressed_fit.decompress(&ships, &modules).unwrap();
        let mut expected = Fit::new("", SHIP.deref());
        expected.add_module(MICROWARPDRIVE.deref());
        expected.should().eq(fit);
    }
}
