use fitting_engine::ship::Ship;
use macros::generate_all_data;
use std::borrow::Cow;

generate_all_data!();

pub fn get_all() -> &'static [Ship<'static>] {
    &ALL_SHIPS
}
