# Test coverage of the `augmented-audio` repository
Currently, test-coverage is captured by Linux builds through `cargo-tarpaulin`.

[You can see a detailed test coverage report here.](https://coveralls.io/github/yamadapc/augmented-audio?branch=master)

Coverage is low. The aggregate test-coverage for all the files on the repository is around 60%. The reason for this is
that large amounts of the repository are GUI code using `iced`. There's no convenient way to test this code. There is
code on this repository which is extremely experimental as well and simply has no tests.

For the `crates/augmented` directory, which contains all code that is meant for re-use, test coverage is around 80%.

This is despite a lot of code in that directory doing DSP. Unit-tests for DSP are not the most meaningful and are
generally smoke tests. There are snapshot tests, described in `docs/monorepo-tooling/SNAPSHOT_TESTING.md`.
