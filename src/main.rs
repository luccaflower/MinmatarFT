use sde_parser::ParserArgument;
use std::fs::File;
use std::path::Path;

fn static_dir_path() -> &'static Path {
    Path::new("__static_data")
}

fn idk<_0, _1, _2>() {}

fn main() {
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
    sde_parser::parse(args).unwrap();
}
