use crate::module::Module;
use crate::ship::Ship;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Deref;
use crate::stats::fitting::*;
use crate::stats::{Stat, ModificationType};

#[derive(Debug, Clone)]
pub struct Fit<'a> {
    ship: &'a Ship<'a>,
    high_slots: Box<[Option<&'a Module<'a>>]>,
    med_slots: Box<[Option<&'a Module<'a>>]>,
    low_slots: Box<[Option<&'a Module<'a>>]>,
}

impl<'a> Fit<'a> {
    pub fn new(ship: &'a Ship) -> Self {
        fn generate_empty<'a>(size: u8) -> Box<[Option<&'a Module<'a>>]> {
            (0..size)
                .into_iter()
                .map(|_| None)
                .collect::<Vec<Option<&'a Module>>>()
                .into_boxed_slice()
        }
        Self {
            ship,
            high_slots: generate_empty(ship.high_slots),
            med_slots: generate_empty(ship.med_slots),
            low_slots: generate_empty(ship.low_slots),
        }
    }

    pub fn compress(self) -> CompressedFit<'a> {
        CompressedFit::new(
            self.ship.name.deref(),
            self.convert_slot(self.high_slots.deref()),
            self.convert_slot(self.med_slots.deref()),
            self.convert_slot(self.low_slots.deref()),
        )
    }

    fn convert_slot(&'a self, slots: &'a [Option<&'a Module>]) -> Vec<String> {
        slots
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .map(|x| x.name.to_string())
            .collect()
    }
}

impl<'a> Into<CompressedFit<'a>> for Fit<'a> {
    fn into(self) -> CompressedFit<'a> {
        self.compress()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedFit<'a> {
    ship: Cow<'a, str>,
    high_slots: Vec<Cow<'a, str>>,
    med_slots: Vec<Cow<'a, str>>,
    low_slots: Vec<Cow<'a, str>>,
}

impl<'a> CompressedFit<'a> {
    pub fn new<
        O: Into<Cow<'a, str>>,
        AI: Into<Cow<'a, str>>,
        A: IntoIterator<Item = AI>,
        BI: Into<Cow<'a, str>>,
        B: IntoIterator<Item = BI>,
        CI: Into<Cow<'a, str>>,
        C: IntoIterator<Item = CI>,
    >(
        ship: O,
        high_slots: A,
        med_slots: B,
        low_slots: C,
    ) -> Self {
        Self {
            ship: ship.into(),
            high_slots: high_slots.into_iter().map(|x| x.into()).collect(),
            med_slots: med_slots.into_iter().map(|x| x.into()).collect(),
            low_slots: low_slots.into_iter().map(|x| x.into()).collect(),
        }
    }

    pub fn uncompress(
        &self,
        ships: &'a HashMap<&'a str, Ship<'a>>,
        modules: &'a HashMap<&'a str, Module<'a>>,
    ) -> Option<Fit> {
        fn create_module_lists<'a>(
            names: &'a [Cow<'a, str>],
            max: u8,
            modules: &'a HashMap<&'a str, Module<'a>>,
        ) -> Option<Box<[Option<&'a Module<'a>>]>> {
            if names.len() > max as usize {
                return None;
            }
            let nones_to_add = (names.len() as u8) - max;
            Some(
                names
                    .iter()
                    .map(|x| modules.get(x.deref()))
                    .chain((0..nones_to_add).map(|_| None))
                    .collect::<Vec<Option<&Module>>>()
                    .into_boxed_slice(),
            )
        }
        let ship = ships.get(self.ship.deref())?;
        let high_slots = create_module_lists(&self.high_slots, ship.high_slots, modules)?;
        let med_slots = create_module_lists(&self.med_slots, ship.med_slots, modules)?;
        let low_slots = create_module_lists(&self.low_slots, ship.low_slots, modules)?;
        Some(Fit {
            ship,
            high_slots,
            med_slots,
            low_slots,
        })
    }
}
