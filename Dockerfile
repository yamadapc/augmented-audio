FROM ubuntu
RUN apt-get update && apt-get install -y sudo curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > init-rustup.sh && chmod +x ./init-rustup.sh && ./init-rustup.sh -y

ADD ./scripts/ /app/scripts
WORKDIR /app
RUN scripts/install-ubuntu-dependencies.sh

ADD . /app/
