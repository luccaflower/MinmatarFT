use sde_parser::{InputSdeData, ParserArgument};
use std::borrow::Cow;
use std::env::current_dir;
use std::fs::{create_dir_all, File};
use std::io::{Cursor, Read, Write};
use std::ops::Deref;
use std::path::Path;
use std::{fs, io};
use zip::read::ZipFile;

pub struct SdeProvider<'a> {
    cached_dir_name: Option<Cow<'a, str>>,
    url: Cow<'a, str>,
}

impl<'a> SdeProvider<'a> {
    pub fn new() -> Self {
        Self {
            cached_dir_name: None,
            url: Cow::Borrowed(
                "https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip",
            ),
        }
    }

    pub fn cached<T: Into<Cow<'a, str>>>(mut self, dir_name: T) -> Self {
        self.cached_dir_name = Some(dir_name.into());
        self
    }

    pub fn url<T: Into<Cow<'a, str>>>(mut self, url: T) -> Self {
        self.url = url.into();
        self
    }
}

impl SdeProvider<'_> {
    async fn fetch(&self) -> Vec<u8> {
        reqwest::get(self.url.deref())
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap()
            .to_vec()
    }
    pub async fn execute(self) -> InputSdeData {
        match &self.cached_dir_name {
            None => {
                let bytes = Cursor::new(self.fetch().await);
                let mut zip = zip::ZipArchive::new(bytes).unwrap();

                fn into(mut z: ZipFile) -> Cursor<Vec<u8>> {
                    let mut buffer = Vec::with_capacity(z.size() as usize);
                    z.read_to_end(&mut buffer).unwrap();
                    Cursor::new(buffer)
                }

                let type_ids =
                    into(zip.by_name("sde/fsd/typeIDs.yaml").unwrap());
                let group_ids =
                    into(zip.by_name("sde/fsd/groupIDs.yaml").unwrap());
                let category_ids =
                    into(zip.by_name("sde/fsd/categoryIDs.yaml").unwrap());
                let dogma_attributes =
                    into(zip.by_name("sde/fsd/dogmaAttributes.yaml").unwrap());
                let type_dogma =
                    into(zip.by_name("sde/fsd/typeDogma.yaml").unwrap());
                ParserArgument::new(
                    type_ids,
                    group_ids,
                    category_ids,
                    dogma_attributes,
                    type_dogma,
                )
                .into()
            }
            Some(dir_name) => {
                let dir = Path::new(dir_name.deref());
                if !dir.exists() {
                    create_dir_all(dir).unwrap();
                }
                let zip_path = dir.join("sde.zip");
                if !zip_path.exists() {
                    let bytes = self.fetch().await;
                    let mut zip_file = File::create(&zip_path).unwrap();
                    zip_file.write_all(bytes.as_slice()).unwrap();
                    zip_file.flush().unwrap();
                }
                let extract_dir = dir.join("sde");
                if !extract_dir.exists() {
                    let mut archive =
                        zip::ZipArchive::new(File::open(&zip_path).unwrap())
                            .unwrap();
                    for i in 0..archive.len() {
                        let mut file = archive.by_index(i).unwrap();
                        let outpath = match file.enclosed_name() {
                            Some(path) => path.to_owned(),
                            None => continue,
                        };

                        let outpath =
                            current_dir().unwrap().join(&dir).join(outpath);
                        if (&*file.name()).ends_with('/') {
                            fs::create_dir_all(&outpath).unwrap();
                        } else {
                            if let Some(p) = outpath.parent() {
                                let p =
                                    current_dir().unwrap().join(&dir).join(p);
                                if !p.exists() {
                                    fs::create_dir_all(&p).unwrap();
                                }
                            }
                            let mut outfile =
                                fs::File::create(&outpath).unwrap();
                            io::copy(&mut file, &mut outfile).unwrap();
                        }
                    }
                }
                ParserArgument::new(
                    File::open(
                        dir.join("sde").join("fsd").join("typeIDs.yaml"),
                    )
                    .unwrap(),
                    File::open(
                        dir.join("sde").join("fsd").join("groupIDs.yaml"),
                    )
                    .unwrap(),
                    File::open(
                        dir.join("sde").join("fsd").join("categoryIDs.yaml"),
                    )
                    .unwrap(),
                    File::open(
                        dir.join("sde")
                            .join("fsd")
                            .join("dogmaAttributes.yaml"),
                    )
                    .unwrap(),
                    File::open(
                        dir.join("sde").join("fsd").join("typeDogma.yaml"),
                    )
                    .unwrap(),
                )
                .into()
            }
        }
    }
}
