FROM rust:1.40.0 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY ./crates ./crates
COPY ./migrations ./migrations
RUN cargo build --release


COPY ./ ./
RUN cargo install --path .

FROM debian:buster-slim
WORKDIR /usr/src/app

COPY --from=builder /app/src/agregator/target/release/agregator ./
COPY --from=builder /usr/src/app/migration ./migration

CMD ["agregator"]