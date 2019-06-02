ARG RUST_IMAGE=rust:1.35-slim-stretch
FROM $RUST_IMAGE as build
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /src
RUN mkdir -p src && touch src/lib.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked
RUN cargo build --frozen --release --target=x86_64-unknown-linux-musl
COPY src src
RUN cargo build --frozen --release --target=x86_64-unknown-linux-musl

FROM scratch
COPY --from=build \
    /src/target/x86_64-unknown-linux-musl/release/the-winter-of-our-disconnect \
    /the-winter-of-our-disconnect
ENTRYPOINT ["/the-winter-of-our-disconnect"]
