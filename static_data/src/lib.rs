use macros::generate_all_data;
use std::borrow::Cow;
use domain::ship::Ship;

generate_all_data!();

pub fn get_all() -> &'static [Ship] {
    &ALL_SHIPS
}
