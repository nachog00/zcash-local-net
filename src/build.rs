use std::{
    fs::File,
    io::{BufReader, Write},
    path::PathBuf,
};

use cargo_readme::generate_readme;

pub fn main() {
    println!("cargo::rerun-if-changed=src/lib.rs");
    let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let source_path = crate_dir.join("src/lib.rs");
    let source_file = File::open(&source_path).expect("to find source source_file");
    let mut reader = BufReader::new(source_file);
    let output = generate_readme(&crate_dir, &mut reader, None, false, false, false, false)
        .expect("to get proper output");

    let mut readme_file =
        File::create(crate_dir.join("README.md")).expect("to create readme file correctly");
    readme_file
        .write_all(output.as_bytes())
        .expect("readme file to update correctly");
    println!("Generated readme:/n{}", output)
}
