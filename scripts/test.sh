#!/usr/bin/env bash

cargo nextest run \
    --verbose \
    --workspace \
    --features story \
    --exclude augmented-ui \
    --exclude midir \
    --exclude assert-no-alloc