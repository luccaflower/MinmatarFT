use crate::module::Module;
use crate::ship::Ship;

#[derive(Debug, Clone)]
pub struct Fit<'a> {
    ship: &'a Ship,
    high_slots: Vec<Option<&'a Module>>,
    med_slots: Vec<Option<&'a Module>>,
    low_slots: Vec<Option<&'a Module>>,
}

impl<'a> Fit<'a> {
    pub fn new(ship: &'a Ship) -> Self {
        Self {
            ship,
            high_slots: Vec::with_capacity(ship.high_slots.clone() as usize),
            med_slots: Vec::with_capacity(ship.med_slots.clone() as usize),
            low_slots: Vec::with_capacity(ship.low_slots.clone() as usize),
        }
    }
}
