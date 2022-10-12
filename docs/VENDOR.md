# Forked libraries log

This repository depends on forks of some dependencies. This file keeps track of the changes we
have made and upstream issues/PRs that are pending so we can use the mainline versions.

## libsamplerate-sys

* Fix iOS compilation - https://github.com/Prior99/libsamplerate-sys/issues/18

## coreaudio-sys

* Fix issues with iOS input callback - https://github.com/RustAudio/coreaudio-rs/pull/91

## cpal

* Upgrade coreaudio-sys to 0.11 - https://github.com/RustAudio/cpal/pull/706

## assert-no-alloc

* Printing back-traces, can be replaced

## vst

* Using unreleased master version