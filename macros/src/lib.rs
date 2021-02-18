mod domain_impl;

use crate::domain_impl::ship::ShipWrapper;
use crate::domain_impl::slice::SliceWrapper;
use proc_macro::TokenStream;
use sde_parser::ParserArgument;
use std::env::current_dir;
use std::fs::{create_dir, File};
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::{fs, io};
use tokio::runtime::Runtime;

fn static_dir_path() -> &'static Path {
    Path::new("__static_data")
}

async fn download_static_data() {
    if !static_dir_path().exists() {
        create_dir(static_dir_path()).unwrap();
    }
    let zip_path = static_dir_path().join("sde.zip");
    if !zip_path.exists() {
        let download_link =
            "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip";
        let bytes = reqwest::get(download_link)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        let mut zip_file = File::create(&zip_path).unwrap();
        zip_file.write_all(bytes.deref()).unwrap();
        zip_file.flush().unwrap();
    };
    if !static_dir_path().join("sde").exists() {
        let mut archive = zip::ZipArchive::new(File::open(&zip_path).unwrap()).expect("5");
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).expect("4");
            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            let outpath = current_dir().unwrap().join(static_dir_path()).join(outpath);
            if (&*file.name()).ends_with('/') {
                fs::create_dir_all(&outpath).expect("3");
            } else {
                if let Some(p) = outpath.parent() {
                    let p = current_dir().unwrap().join(static_dir_path()).join(p);
                    if !p.exists() {
                        fs::create_dir_all(&p).expect("2");
                    }
                }
                let mut outfile = fs::File::create(&outpath)
                    .expect(format!("failed to write to {:?}", outpath).as_str());
                io::copy(&mut file, &mut outfile).expect("1");
            }
        }
    }
}

#[proc_macro]
pub fn generate_all_data(_: TokenStream) -> TokenStream {
    let runtime = Runtime::new().unwrap();
    runtime.block_on(download_static_data());
    let args = ParserArgument::new(
        File::open(
            static_dir_path()
                .join("sde")
                .join("fsd")
                .join("typeIDs.yaml"),
        )
        .unwrap(),
        File::open(
            static_dir_path()
                .join("sde")
                .join("fsd")
                .join("groupIDs.yaml"),
        )
        .unwrap(),
        File::open(
            static_dir_path()
                .join("sde")
                .join("fsd")
                .join("categoryIDs.yaml"),
        )
        .unwrap(),
        File::open(
            static_dir_path()
                .join("sde")
                .join("fsd")
                .join("dogmaAttributes.yaml"),
        )
        .unwrap(),
        File::open(
            static_dir_path()
                .join("sde")
                .join("fsd")
                .join("typeDogma.yaml"),
        )
        .unwrap(),
    );
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
