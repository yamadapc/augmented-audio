# Version 0.1.3
#
#   docker build -t yamadapc/augmented-audio-linux-ci:0.1.0 -f ./docker/linux-ci.dockerfile .
#   docker push yamadapc/augmented-audio-linux-ci:0.1.0
#
FROM ubuntu:20.04

RUN apt-get update
RUN apt-get install --fix-missing -y \
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
    libgtksourceview-3.0-dev \
    libwebkit2gtk-4.0-dev \
    libx11-xcb-dev \
    libxcb-icccm4-dev \
    libxcb-dri2-0-dev \
    nvidia-utils-465 \
    libgtk-3-dev

RUN apt-get install -y curl git
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup default stable