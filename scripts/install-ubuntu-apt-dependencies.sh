#!/usr/bin/env bash
set -e
sudo apt-get update
sudo apt-get install --fix-missing -y \
    libasound2-dev \
    libssl-dev \
    cmake \
    libfreetype6-dev \
    expat \
    libexpat1-dev \
    libglib2.0-dev \
    libcairo-dev \
    libpango1.0-dev \
    libatk1.0-dev \
    libgdk-pixbuf2.0-dev \
    libsoup2.4-dev \
    libclang-11-dev \
    libgdk3.0-cil-dev \
    libappindicator3-dev \
    libayatana-appindicator-dev \
    libgtksourceview-3.0-dev \
    libwebkit2gtk-4.0-dev \
    libx11-xcb-dev \
    libxcb-icccm4-dev \
    libxcb-dri2-0-dev \
    libavahi-client-dev \
    nvidia-utils-465 \
    lame \
    libgtk-3-dev
