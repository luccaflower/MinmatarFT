use crate::module_instance::ModuleInstance;
use crate::ship::Ship;
use crate::static_module::StaticModule;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::ops::Deref;

pub type Modules<'a> = Box<[Option<ModuleInstance<'a>>]>;

#[derive(Debug, Clone)]
pub struct Fit<'a> {
    pub ship: &'a Ship<'a>,
    pub high_slots: Modules<'a>,
    pub med_slots: Modules<'a>,
    pub low_slots: Modules<'a>,
}

impl<'a> Fit<'a> {
    pub fn new(ship: &'a Ship) -> Self {
        fn generate_empty<'a>(size: u8) -> Modules<'a> {
            (0..size)
                .into_iter()
                .map(|_| None)
                .collect::<Vec<Option<ModuleInstance>>>()
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

    fn convert_slot(&'a self, slots: &'a [Option<ModuleInstance<'a>>]) -> Vec<String> {
        slots
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap())
            .map(|x| x.inner_module.name.to_string())
            .collect()
    }
    /*
    pub fn calculate_stats(&'a self) -> (Capacitor, Defense, Drone, Fitting, Movement, Sensor) {
        fn maybe_push<T>(v: &mut Vec<&T>, t: &Option<T>) {
            if let Some(val) = t {
                v.push(val)
            }
        }
        self.low_slots
            .iter()
            .chain(self.med_slots.iter())
            .chain(self.high_slots.iter())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .fold(
                (vec![], vec![], vec![], vec![], vec![], vec![]),
                |(
                    mut capacitor_vec,
                    mut defense_vec,
                    mut drone_vec,
                    fitting_vec,
                    movement_vec,
                    sensor_vec,
                ),
                 x| {
                    let StaticModule {
                        passive_fitting,
                        active_fitting,
                        passive_capacitor,
                        active_capacitor,
                        passive_defense,
                        active_defense,
                        passive_movement,
                        active_movement,
                        passive_sensor,
                        active_sensor,
                        passive_drone,
                        active_drone,
                        ..
                    } = x;
                    maybe_push(&mut capacitor_vec, passive_capacitor);
                    maybe_push(&mut capacitor_vec, active_capacitor);
                    maybe_push(&mut defense_vec, passive_defense);
                    maybe_push(&mut defense_vec, active_defense);
                    maybe_push(&mut drone_vec, passive_drone);
                    maybe_push(&mut drone_vec, active_drone);
                    maybe_push(&mut drone_vec, passive_drone);
                    maybe_push(&mut drone_vec, active_drone);
                    (
                        capacitor_vec,
                        defense_vec,
                        drone_vec,
                        fitting_vec,
                        movement_vec,
                        sensor_vec,
                    )
                },
            );
    }
     */
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
            let nones_to_add = (names.len() as u8) - max;
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
