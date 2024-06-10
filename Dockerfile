# Building Stage
FROM rust:1.77.2 as cargo-builder

ENV  RUSTUP_DIST_SERVER="https://rsproxy.cn" \
    RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

RUN mkdir ~/.cargo \
    && echo -e '[source.crates-io]' >> ~/.cargo/config.toml \
    && echo 'replace-with = "rsproxy-sparse"' >> ~/.cargo/config.toml \
    && echo '[source.rsproxy]' >> ~/.cargo/config.toml \
    && echo 'registry = "https://rsproxy.cn/crates.io-index"' >> ~/.cargo/config.toml \
    && echo '[source.rsproxy-sparse]' >> ~/.cargo/config.toml \
    && echo 'registry = "sparse+https://rsproxy.cn/index/"' >> ~/.cargo/config.toml \
    && echo '[registries.rsproxy]' >> ~/.cargo/config.toml \
    && echo 'index = "https://rsproxy.cn/crates.io-index"' >> ~/.cargo/config.toml \
    && echo '[net]' >> ~/.cargo/config.toml \
    && echo 'git-fetch-with-cli = true' >> ~/.cargo/config.toml

RUN apt-get update 
RUN apt-get install musl-tools -y 
RUN rustup target add x86_64-unknown-linux-musl 

WORKDIR /app
COPY Cargo.toml Cargo.toml
RUN mkdir src
RUN echo 'fn main() { println!("If you see this, It means that the building has failed") }' > src/main.rs
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl 
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/rs-aksem*
# RUN cargo build --release
# RUN rm -f target/release/deps/rs-aksem*
# RUN cargo build --release
# RUN cargo install --path .
COPY . . 
RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl 

# Filnal Stage
FROM alpine:latest
COPY --from=cargo-builder /app/target/x86_64-unknown-linux-musl/release/rs-aksem /usr/local/bin/rs-aksem
CMD [ "rs-aksem" ]