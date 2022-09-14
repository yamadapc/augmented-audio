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
use crate::manifests::CargoToml;
use crate::services::ListCratesService;
use cmd_lib::{run_cmd, run_fun, spawn};

pub fn run_snapshot_tests(_path: &str, manifest: CargoToml, update_snapshots: bool) {
    let crate_name = manifest.package.name;
    log::info!("Running snapshot tests for {}", crate_name);
    let metadata = manifest.package.metadata.unwrap();
    let examples = metadata.augmented.unwrap().processor_examples.unwrap();

    for example in examples {
        run_example_snapshot_tests(&crate_name, &example, update_snapshots);
    }
}

fn run_example_snapshot_tests(crate_name: &str, example: &str, update_snapshots: bool) {
    spawn!(cargo build --package ${crate_name} --release --example ${example})
        .unwrap()
        .wait()
        .expect("Failed to build example");

    run_cmd!(mkdir -p test/snapshots/${crate_name}/).unwrap();
    #[cfg(target_os = "macos")]
    {
        spawn!(cargo run --target x86_64-apple-darwin --package ${crate_name} --release --example ${example} -- --input-file ./input-files/C3-loop.mp3 --output-file test/snapshots/${crate_name}/${example}.tmp.wav)
            .unwrap()
            .wait()
            .expect("Failed to run example");
    }
    #[cfg(not(target_os = "macos"))]
    {
        spawn!(cargo run --package ${crate_name} --release --example ${example} -- --input-file ./input-files/C3-loop.mp3 --output-file test/snapshots/${crate_name}/${example}.tmp.wav)
            .unwrap()
            .wait()
            .expect("Failed to run example");
    }

    let md5_commit =
        run_fun!(md5 -q test/snapshots/${crate_name}/${example}.wav).unwrap_or_else(|_| "".into());
    let md5_test = run_fun!(md5 -q test/snapshots/${crate_name}/${example}.tmp.wav)
        .unwrap_or_else(|_| "".into());

    if update_snapshots {
        if md5_test != md5_commit {
            log::warn!("Updating snapshot {}/{}", crate_name, example,);
            run_cmd!(mv test/snapshots/${crate_name}/${example}.tmp.wav test/snapshots/${crate_name}/${example}.wav).unwrap();
        }
    } else if md5_test != md5_commit {
        log::error!(
            "Test failed for {}/{}\n   Expected: {:?} Received: {:?}",
            crate_name,
            example,
            md5_commit,
            md5_test
        );
        std::process::exit(1);
    } else {
        run_cmd!(rm test/snapshots/${crate_name}/${example}.tmp.wav).unwrap();
    }
}

pub fn run_all_snapshot_tests(list_crates_service: ListCratesService, update_snapshots: bool) {
    let augmented_crates = list_crates_service.find_augmented_crates();
    augmented_crates
        .into_iter()
        .filter(|(_, manifest)| manifest.has_snapshot_tests())
        .for_each(|(path, manifest)| run_snapshot_tests(&path, manifest, update_snapshots));
}
