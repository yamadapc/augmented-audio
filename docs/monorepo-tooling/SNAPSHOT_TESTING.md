# Snapshot Testing
Audio Processors can be snapshot tested.

## Running snapshot tests
```
./scripts/dev.sh test-snapshots
```

The `augmented-dev-cli` will find crates that declare audio processor examples.
It's expected that these examples make use of the `audio-processor-standalone` library and declare a CLI program that
can be invoked.

The examples will then be run to generate output files and these files will be compared with older stored versions.

## Updating snapshots
```
./scripts/dev.sh test-snapshots --update-snapshots
```

## Declaring audio processor examples
Add the following to `Cargo.toml` under the `package.metadata.augmented` section:

```toml
[package.metadata.augmented]
processor_examples = ["example_name"]
```