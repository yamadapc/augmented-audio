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
    spawn!(cargo run --package ${crate_name} --release --example ${example} -- --input-file ./input-files/C3-loop.mp3 --output-file test/snapshots/${crate_name}/${example}.tmp.wav)
        .unwrap()
        .wait()
        .expect("Failed to run example");

    let md5_commit =
        run_fun!(md5 -q test/snapshots/${crate_name}/${example}.wav).unwrap_or("".into());
    let md5_test =
        run_fun!(md5 -q test/snapshots/${crate_name}/${example}.tmp.wav).unwrap_or("".into());

    if update_snapshots {
        if md5_test != md5_commit {
            log::warn!("Updating snapshot {}/{}", crate_name, example,);
            run_cmd!(mv test/snapshots/${crate_name}/${example}.tmp.wav test/snapshots/${crate_name}/${example}.wav).unwrap();
        }
    } else {
        if md5_test != md5_commit {
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
}

pub fn run_all_snapshot_tests(list_crates_service: ListCratesService, update_snapshots: bool) {
    let augmented_crates = list_crates_service.find_augmented_crates();
    augmented_crates
        .into_iter()
        .filter(|(_, manifest)| manifest.has_snapshot_tests())
        .for_each(|(path, manifest)| run_snapshot_tests(&path, manifest, update_snapshots));
}
