#!/usr/bin/env bash

cargo test --workspace \
      --target-dir target-linux \
      --features story \
      --exclude augmented-ui \
      --exclude audiounit \
      --exclude assert-no-alloc \
      --exclude basedrop \
      --exclude midir \
      --exclude gfx \
      --exclude iced \
      --exclude iced-baseview \
      --exclude iced_audio \
      --exclude libloading \
      --exclude lyon \
      --exclude pathfinder \
      --exclude piet \
      --exclude plotters \
      --exclude skribo \
      --exclude vst \
      --exclude example-iced-xcode-integration \
      --exclude avfaudio-sys \
      --exclude recording_buddy \
      --exclude darwin-webkit \
      --exclude webview-holder \
      --exclude macos-bundle-resources
