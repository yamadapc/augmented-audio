#!/usr/bin/env bash
set -x
swiftformat ./Sequencer/**/*.swift
swiftformat ./SequencerUITests/**/*.swift
swiftformat ./Sequencer\ AudioUnit/**/*.swift
swiftformat ./SequencerUI/**/*.swift
swiftformat ./Sequencer\ Mac/**/*.swift
swiftformat ./Sequencer\ MacUITests/**/*.swift
swiftformat ./SequencerEngine/**/*.swift
