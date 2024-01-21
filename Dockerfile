ARG BASE_IMAGE=rust:slim-bookworm

FROM $BASE_IMAGE as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM $BASE_IMAGE as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM $BASE_IMAGE as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN rustup target add x86_64-unknown-linux-musl
# `cargo build` doesn't work in static linking, need `cargo install`
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/bingokta .
HEALTHCHECK CMD curl --fail http://localhost:8080/ping || exit 1
CMD ["./bingokta"]