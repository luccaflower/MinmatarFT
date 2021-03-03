use crate::fit_stats::FitStats;
use crate::module_instance::ModuleInstance;
use crate::ship::Ship;
use crate::static_module::{ModuleSlot, StaticModule};
use crate::stats::Stat;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Deref;

pub type Modules<'a> = Box<[Option<ModuleInstance<'a>>]>;

#[derive(Debug, Clone)]
pub struct Fit<'a> {
    pub name: Cow<'a, str>,
    pub ship: &'a Ship<'a>,
    pub high_slots: Modules<'a>,
    pub med_slots: Modules<'a>,
    pub low_slots: Modules<'a>,
}

impl<'a> Fit<'a> {
    pub fn rename<S: Into<Cow<'a, str>>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn new<S: Into<Cow<'a, str>>>(name: S, ship: &'a Ship) -> Self {
        fn generate_empty<'a>(size: u8) -> Modules<'a> {
            (0..size)
                .into_iter()
                .map(|_| None)
                .collect::<Vec<Option<ModuleInstance>>>()
                .into_boxed_slice()
        }
        Self {
            name: name.into(),
            ship,
            high_slots: generate_empty(ship.high_slots),
            med_slots: generate_empty(ship.med_slots),
            low_slots: generate_empty(ship.low_slots),
        }
    }

    pub fn compress(self) -> CompressedFit<'a> {
        let high_slots = self.convert_slot(self.high_slots.deref());
        let med_slots = self.convert_slot(self.med_slots.deref());
        let low_slots = self.convert_slot(self.low_slots.deref());
        let Fit { name, ship, .. } = self;
        CompressedFit::new(name, ship.name.deref(), high_slots, med_slots, low_slots)
    }

    pub fn add_module(&mut self, module: &'a StaticModule<'a>) -> bool {
        let modules = match &module.module_slot {
            ModuleSlot::High => &mut self.high_slots,
            ModuleSlot::Med => &mut self.med_slots,
            ModuleSlot::Low => &mut self.low_slots,
            ModuleSlot::Rig => unimplemented!(),
        };
        for i in 0..modules.len() {
            if modules[i].is_none() {
                modules[i] = Some(ModuleInstance::new(module));
                return true;
            }
        }
        false
    }

    fn convert_slot(&'a self, slots: &'a [Option<ModuleInstance<'a>>]) -> Vec<String> {
        slots
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap())
            .map(|x| x.inner_module.name.to_string())
            .collect()
    }
    pub fn calculate_stats(&self) -> FitStats {
        fn maybe_push<'a, T>(v: &mut Vec<&'a T>, val: &'a Option<T>) {
            let val = val.as_ref();
            if let Some(val) = val {
                v.push(val)
            }
        }
        let (fitting, capacitor, defense, movement, sensor, drone) = self
            .high_slots
            .iter()
            .chain(self.med_slots.iter())
            .chain(self.low_slots.iter())
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap())
            .fold(
                (vec![], vec![], vec![], vec![], vec![], vec![]),
                |(
                    mut fitting_vec,
                    mut capacitor_vec,
                    mut defense_vec,
                    mut movement_vec,
                    mut sensor_vec,
                    mut drone_vec,
                ),
                 x| {
                    let (fitting, capacitor, defense, movement, sensor, drone) = x.modifications();
                    maybe_push(&mut fitting_vec, fitting);
                    maybe_push(&mut capacitor_vec, capacitor);
                    maybe_push(&mut defense_vec, defense);
                    maybe_push(&mut movement_vec, movement);
                    maybe_push(&mut sensor_vec, sensor);
                    maybe_push(&mut drone_vec, drone);
                    (
                        fitting_vec,
                        capacitor_vec,
                        defense_vec,
                        movement_vec,
                        sensor_vec,
                        drone_vec,
                    )
                },
            );
        let fitting = self.ship.fitting_stats.apply(fitting);
        let capacitor = self.ship.capacitor_stats.apply(capacitor);
        let defense = self.ship.defensive_stats.apply(defense);
        let movement = self.ship.movement_stats.apply(movement);
        let sensor = self.ship.sensor_stats.apply(sensor);
        let drone = self.ship.drone_stats.apply(drone);
        FitStats::new(fitting, capacitor, defense, movement, sensor, drone)
    }
}

impl<'a> Into<CompressedFit<'a>> for Fit<'a> {
    fn into(self) -> CompressedFit<'a> {
        self.compress()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedFit<'a> {
    pub name: Cow<'a, str>,
    pub ship: Cow<'a, str>,
    pub high_slots: Vec<Cow<'a, str>>,
    pub med_slots: Vec<Cow<'a, str>>,
    pub low_slots: Vec<Cow<'a, str>>,
}

impl<'a> CompressedFit<'a> {
    pub fn new<
        N: Into<Cow<'a, str>>,
        O: Into<Cow<'a, str>>,
        AI: Into<Cow<'a, str>>,
        A: IntoIterator<Item = AI>,
        BI: Into<Cow<'a, str>>,
        B: IntoIterator<Item = BI>,
        CI: Into<Cow<'a, str>>,
        C: IntoIterator<Item = CI>,
    >(
        name: N,
        ship: O,
        high_slots: A,
        med_slots: B,
        low_slots: C,
    ) -> Self {
        Self {
            name: name.into(),
            ship: ship.into(),
            high_slots: high_slots.into_iter().map(|x| x.into()).collect(),
            med_slots: med_slots.into_iter().map(|x| x.into()).collect(),
            low_slots: low_slots.into_iter().map(|x| x.into()).collect(),
        }
    }

    pub fn decompress(
        &self,
        ships: &'a HashMap<&'a str, Ship<'a>>,
        modules: &'a HashMap<&'a str, StaticModule<'a>>,
    ) -> Option<Fit> {
        fn create_module_lists<'a>(
            names: &'a [Cow<'a, str>],
            max: u8,
            modules: &'a HashMap<&'a str, StaticModule<'a>>,
        ) -> Option<Box<[Option<ModuleInstance<'a>>]>> {
            if names.len() > max as usize {
                return None;
            }
            let nones_to_add = max - (names.len() as u8);
            Some(
                names
                    .iter()
                    .map(|x| modules.get(x.deref()).map(|x| ModuleInstance::new(x)))
                    .chain((0..nones_to_add).map(|_| None))
                    .collect::<Vec<Option<ModuleInstance>>>()
                    .into_boxed_slice(),
            )
        }
        let ship = ships.get(self.ship.deref())?;
        let high_slots = create_module_lists(&self.high_slots, ship.high_slots, modules)?;
        let med_slots = create_module_lists(&self.med_slots, ship.med_slots, modules)?;
        let low_slots = create_module_lists(&self.low_slots, ship.low_slots, modules)?;
        Some(Fit {
            name: Cow::Borrowed(""),
            ship,
            high_slots,
            med_slots,
            low_slots,
        })
    }
}

#[cfg(test)]
mod tests {
    mod any_fit {
        fn has_a_slot_layout_matching_its_associated_ship() {}
    }

    mod an_empty_fit {}

    mod a_nonempty_fit {}
}
