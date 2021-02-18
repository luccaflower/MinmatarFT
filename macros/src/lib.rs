mod domain_impl;

use crate::domain_impl::ship::ShipWrapper;
use crate::domain_impl::slice::SliceWrapper;
use proc_macro::TokenStream;
use tokio::runtime::Runtime;
use sde_provider::SdeProvider;

#[proc_macro]
pub fn generate_all_data(_: TokenStream) -> TokenStream {
    let runtime = Runtime::new().unwrap();
    let args = runtime.block_on(SdeProvider::new().cached("__static_data").execute());
    let result = sde_parser::parse(args).unwrap();
    let ships = result
        .into_iter()
        .map(|(_, x)| ShipWrapper::new(x))
        .collect::<Vec<ShipWrapper>>()
        .into_boxed_slice();
    let length = ships.len();
    let a = SliceWrapper::new(ships);

    TokenStream::from(quote::quote! {
        static ALL_SHIPS: [fitting_engine::ship::Ship; #length] = #a;
    })
}
