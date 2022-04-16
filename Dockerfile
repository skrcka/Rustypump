FROM debian
RUN apt-get update && apt install -y curl gcc-aarch64-linux-gnu g++-aarch64-linux-gnu gcc
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup target add aarch64-unknown-linux-gnu
WORKDIR /project