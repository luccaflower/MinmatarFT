pub mod manual_memory;

use crate::manual_memory::{
    create_pointer, drop_pointer, use_pointer, use_pointer_mut,
};
use fitting_engine::fit::Fit;
use fitting_engine::ship::Ship;
use js_sys::Array;
use serde_json::Error;
use std::borrow::Cow;
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn all_ship_names() -> Array {
    static_data::get_all()
        .iter()
        .map(|x| x.name.deref())
        .map(|x| JsValue::from(x))
        .collect::<Array>()
}

#[wasm_bindgen]
pub fn all_ships() -> Array {
    static_data::get_all()
        .iter()
        .map(|x| serde_json::to_string(x).unwrap())
        .map(|x| js_sys::JSON::parse(x.as_str()).unwrap())
        .collect()
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

#[wasm_bindgen]
pub fn fetch_ship_by_name(name: String) -> JsValue {
    fn inner(name: String) -> Option<JsValue> {
        let json = serde_json::to_string(
            static_data::get_all().iter().find(|x| x.name == name)?,
        )
        .ok()?;
        Some(js_sys::JSON::parse(json.as_str()).ok()?)
    }
    inner(name).unwrap_or(JsValue::null())
}

#[wasm_bindgen]
pub fn compress_fit(fit: u64) -> JsValue {
    let fit = use_pointer::<Fit>(fit).clone().compress();
    match serde_json::to_string(&fit) {
        Ok(x) => js_sys::JSON::parse(x.as_str()).unwrap_or(JsValue::null()),
        Err(_) => JsValue::null(),
    }
}
