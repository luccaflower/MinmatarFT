mod domain_impl;

use crate::domain_impl::ship::ShipWrapper;
use crate::domain_impl::slice::SliceWrapper;
use fitting_engine::ship::Ship;
use proc_macro::TokenStream;
use sde_provider::SdeProvider;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use tokio::runtime::Runtime;

#[proc_macro]
pub fn generate_all_data(_: TokenStream) -> TokenStream {
    let static_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("__static_data");
    let json_dir = static_dir.join("json");
    let ships = if json_dir.exists() {
        serde_json::from_reader::<_, Vec<Ship>>(File::open(json_dir.join("ships.json")).unwrap())
            .unwrap()
            .into_boxed_slice()
    } else {
        create_dir_all(&json_dir).unwrap();
        let runtime = Runtime::new().unwrap();
        let args = runtime.block_on(
            SdeProvider::new()
                .cached(static_dir.to_string_lossy())
                .execute(),
        );
        let result = sde_parser::parse(args).unwrap();
        let mut file = File::create(json_dir.join("ships.json")).unwrap();
        let json =
            serde_json::to_string(&result.iter().map(|(_, x)| x).collect::<Vec<&Ship>>()).unwrap();
        file.write_all(json.as_bytes()).unwrap();
        file.flush().unwrap();
        result
            .into_iter()
            .map(|(_, x)| x)
            .collect::<Vec<Ship>>()
            .into_boxed_slice()
    };
    let length = ships.len();
    let a = ships
        .to_vec()
        .into_iter()
        .map(|x| ShipWrapper::new(x))
        .collect::<Vec<ShipWrapper>>()
        .into_boxed_slice();
    let a = SliceWrapper::new(a);
    TokenStream::from(quote::quote! {
        static ALL_SHIPS: [fitting_engine::ship::Ship<'static>; #length] = #a;
    })
}
