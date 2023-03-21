# Test coverage of the `augmented-audio` repository
Currently, test-coverage is captured by Linux builds through `cargo-tarpaulin`.

[You can see a detailed test coverage report here.](https://coveralls.io/github/yamadapc/augmented-audio?branch=master)

Coverage is low. The aggregate test-coverage for all the files on the repository is around 60%.

In the metric and report above though, UI code (`crates/apps/\*/src/ui/`) is excluded. These are
mostly iced views.
