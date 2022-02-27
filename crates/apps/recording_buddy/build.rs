fn main() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    uniffi_build::generate_scaffolding(&format!("{}/src/augmented.udl", crate_dir)).unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir.clone())
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!("{}/src/Generated/bindings.h", crate_dir));
}
