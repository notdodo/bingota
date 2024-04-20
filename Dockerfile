ARG BASE_IMAGE=rust:slim-bookworm
ARG PLATFORM=${BUILDPLATFORM}

FROM --platform=${PLATFORM} ${BASE_IMAGE} as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM --platform=${PLATFORM} ${BASE_IMAGE} as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM --platform=${PLATFORM} ${BASE_IMAGE} as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM scratch
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/bingokta .
HEALTHCHECK NONE
CMD ["./bingokta"]