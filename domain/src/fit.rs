use crate::module::Module;
use crate::ship::Ship;

#[derive(Debug, Clone)]
pub struct Fit<'a> {
    ship: &'a Ship,
    high_slots: Box<[Option<&'a Module>]>,
    med_slots: Box<[Option<&'a Module>]>,
    low_slots: Box<[Option<&'a Module>]>,
}

impl<'a> Fit<'a> {
    pub fn new(ship: &'a Ship) -> Self {
        fn generate_empty<'a>(size: u8) -> Box<[Option<&'a Module>]> {
            (0..size).into_iter().map(|_|None).collect::<Vec<Option<&'a Module>>>().into_boxed_slice()
        }
        Self {
            ship,
            high_slots: generate_empty(ship.high_slots.clone()),
            med_slots: generate_empty(ship.med_slots.clone()),
            low_slots: generate_empty(ship.low_slots.clone()),
        }
    }
}
