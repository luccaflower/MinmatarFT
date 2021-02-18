mod domain_impl;

use crate::domain_impl::ship::ShipWrapper;
use crate::domain_impl::slice::SliceWrapper;
use proc_macro::TokenStream;
use tokio::runtime::Runtime;
use sde_provider::SdeProvider;
use std::path::Path;
use std::fs::{File, create_dir_all};
use fitting_engine::ship::Ship;
use std::io::Write;

#[proc_macro]
pub fn generate_all_data(_: TokenStream) -> TokenStream {
    let json_dir = Path::new("__static_data").join("json");
    let ships = if json_dir.exists() {
        serde_json::from_reader::<_, Vec<Ship>>(File::open(json_dir.join("ships.json")).unwrap()).unwrap().into_boxed_slice()
    } else {
        create_dir_all(&json_dir).unwrap();
        let runtime = Runtime::new().unwrap();
        let args = runtime.block_on(SdeProvider::new().cached("__static_data").execute());
        let result = sde_parser::parse(args).unwrap();
        let mut file = File::create(json_dir.join("ships.json")).unwrap();
        let json = serde_json::to_string(&result.iter().map(|(_,x)|x).collect::<Vec<&Ship>>()).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        file.flush().unwrap();
        result.into_iter().map(|(_,x)|x).collect::<Vec<Ship>>().into_boxed_slice()
    };
    let length = ships.len();
    let a = ships.to_vec().into_iter().map(|x|ShipWrapper::new(x)).collect::<Vec<ShipWrapper>>().into_boxed_slice();
    let a = SliceWrapper::new(a);
    TokenStream::from(quote::quote! {
        static ALL_SHIPS: [fitting_engine::ship::Ship<'static>; #length] = #a;
    })
}
