use std::path::PathBuf;

fn main() {
    let _out_dir = PathBuf::from("./generated");
    let bridges = vec!["src/c_api/mod.rs"];
    for path in &bridges {
        println!("cargo:rerun-if-changed={}", path);
    }
}
