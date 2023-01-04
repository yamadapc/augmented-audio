// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn sdk_path() -> Result<String, std::io::Error> {
    use std::process::Command;

    let output = Command::new("xcrun")
        .args(["--show-sdk-path"])
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
    let target = env::var("TARGET").unwrap();
    let target = if target == "aarch64-apple-ios" {
        "arm64-apple-ios"
    } else {
        &target
    };
    let target_arg = format!("--target={}", target);
    let sdk = sdk_path().ok();
    let sdk = sdk.as_ref().map(String::as_ref);
    let mut clang_args = vec![
        "-x",
        "objective-c",
        "-fblocks",
        "-fretain-comments-from-system-headers",
        &target_arg,
    ];
    if let Some(sdk) = sdk {
        clang_args.extend(["-isysroot", sdk]);
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
        .blocklist_function("settimeofday")
        .opaque_type("FndrOpaqueInfo")
        .opaque_type("HFSPlusCatalogFile")
        .opaque_type("HFSCatalogFile")
        .opaque_type("HFSPlusCatalogFolder")
        .opaque_type("HFSCatalogFolder")
        .no_copy("AudioUnitRenderContext")
        .no_debug("AudioUnitRenderContext")
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
