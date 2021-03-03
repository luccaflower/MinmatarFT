pub mod manual_memory;

use crate::manual_memory::{create_pointer, drop_pointer, use_pointer, use_pointer_mut};
use fitting_engine::fit::Fit;
use std::borrow::Cow;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn all_ship_names() -> String {
    let inner = static_data::get_all()
        .iter()
        .map(|x| x.name.deref())
        .collect::<Vec<&str>>();
    serde_json::to_string(&inner).unwrap()
}

#[wasm_bindgen]
pub fn new_fit(ship_name: String) -> Option<u64> {
    let ship = static_data::get_all()
        .into_iter()
        .find(|x| x.name == ship_name)?;
    let fit = Fit::new(Cow::Borrowed("unnamed"), ship);
    Some(create_pointer(fit))
}

#[wasm_bindgen]
pub fn drop_fit(pointer: u64) {
    drop_pointer::<Fit>(pointer)
}

#[wasm_bindgen]
pub fn rename_fit(fit: u64, ship_name: String) {
    let fit = use_pointer_mut::<Fit>(fit);
    fit.rename(ship_name);
}

#[wasm_bindgen]
pub fn get_name_fit(fit: u64) -> String {
    let fit = use_pointer::<Fit>(fit);
    fit.name.to_string()
}
