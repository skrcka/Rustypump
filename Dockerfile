FROM debian
RUN apt-get update && apt install -y curl gcc-arm-linux-gnueabihf gcc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup target add armv7-unknown-linux-gnueabihf
WORKDIR /project