use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("functions");
    let mut f = File::create(&dest_path.join("functions.sql")).unwrap();
    writeln!(
        f,
        "---------------------------------------------------------------------------"
    )
    .unwrap();
    writeln!(
        f,
        "-- This file is autogenerated. Any changes to it will be overwritten"
    )
    .unwrap();
    writeln!(
        f,
        "---------------------------------------------------------------------------"
    )
    .unwrap();
    for entry in fs::read_dir(&dest_path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_name() == "functions.sql" || !entry.file_name().to_str().unwrap().ends_with(".sql") {
            continue;
        }
        let file_name = entry.path();
        if let Some(path) = file_name.to_str() {
            println!("cargo:rerun-if-changed={}", path);
        }

        writeln!(
            f,
            "---------------------------------------------------------------------------"
        )
        .unwrap();
        writeln!(f, "-- from file: {:?}", &entry.file_name()).unwrap();
        writeln!(
            f,
            "---------------------------------------------------------------------------"
        )
        .unwrap();
        f.write_all(&fs::read(file_name).unwrap()).unwrap();
    }
}
