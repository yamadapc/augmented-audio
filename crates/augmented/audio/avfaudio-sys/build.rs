extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn sdk_path() -> Result<String, std::io::Error> {
    use std::process::Command;

    let output = Command::new("xcrun")
        .args(&["--show-sdk-path"])
        .output()?
        .stdout;
    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
    Ok(prefix_str.trim_end().to_string())
}

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=framework=AVFAudio");

    // Tell cargo to invalidate the built crate whenever the wrapper changes

    // See https://github.com/rust-lang/rust-bindgen/issues/1211
    // Technically according to the llvm mailing list, the argument to clang here should be
    // -arch arm64 but it looks cleaner to just change the target.
    let target = std::env::var("TARGET").unwrap();
    let target = if target == "aarch64-apple-ios" {
        "arm64-apple-ios"
    } else {
        &target
    };
    let target_arg = format!("--target={}", target);
    let sdk = sdk_path().ok();
    let sdk = sdk.as_ref().map(String::as_ref);
    let mut clang_args = vec!["-x", "objective-c", "-fblocks", &target_arg];
    if let Some(sdk) = sdk {
        clang_args.extend(&["-isysroot", sdk]);
    }

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .clang_args(&clang_args)
        .objc_extern_crate(true)
        .block_extern_crate(true)
        .generate_block(true)
        .generate_comments(true)
        .rustfmt_bindings(true)
        .blocklist_item("objc_object")
        .blocklist_item("id")
        .blocklist_item("timezone")
        .no_copy("AudioUnitRenderContext")
        .header_contents("AVFAudio.h", "#include<AVFAudio/AVFAudio.h>")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
