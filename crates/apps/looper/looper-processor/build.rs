use std::path::PathBuf;

fn swift_codegen() {}

fn main() {
    let out_dir = PathBuf::from("./generated");
    let bridges = vec!["src/c_api/mod.rs"];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }
}
