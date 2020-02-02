FROM rust:1.40.0 AS build
WORKDIR /usr/src/

# RUN rustup target add x86_64-unknown-linux-musl
RUN USER=root cargo new nyaa_tracker

WORKDIR /usr/src/nyaa_tracker
COPY Cargo.lock Cargo.toml ./
    
RUN cargo build --release

COPY config.json ./

COPY src ./src
RUN cargo install --path .

COPY config.json /usr/local/cargo/bin/config.json

CMD ["/usr/local/cargo/bin/nyaa_tracker"]
# CMD ["ls", "/usr/local/cargo/bin/"]