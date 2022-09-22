FROM ubuntu:20.04

RUN apt-get update && apt-get install -y sudo curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > init-rustup.sh && chmod +x ./init-rustup.sh && ./init-rustup.sh -y
RUN rustup default stable

WORKDIR /app
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get install --fix-missing -y \
    git \
    git-lfs \
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
    libayatana-appindicator3-dev \
    libgtksourceview-3.0-dev \
    libwebkit2gtk-4.0-dev \
    libx11-xcb-dev \
    libxcb-icccm4-dev \
    libxcb-shape0-dev \
    libxcb-dri2-0-dev \
    libxcb-xfixes0-dev \
    libavahi-client-dev \
    lame \
    libgtk-3-dev

RUN . $HOME/.cargo/env && cargo install --version 0.15.2 uniffi_bindgen
RUN . $HOME/.cargo/env && cargo install cargo-nextest
RUN . $HOME/.cargo/env && cargo install cargo-tarpaulin

ADD ./scripts/install-llvm-cov.sh /app/scripts/install-llvm-cov.sh
RUN . $HOME/.cargo/env && ./scripts/install-llvm-cov.sh
RUN . $HOME/.cargo/env && rustup component add llvm-tools-preview

ADD . /app/
