FROM rust:slim

WORKDIR /app

RUN apt-get update
RUN apt-get install -y libpq-dev # diesel_cli dependency
RUN cargo install diesel_cli --no-default-features --features postgres
COPY Cargo.toml  /app/Cargo.toml
COPY Cargo.lock  /app/Cargo.lock
COPY diesel.toml /app/diesel.toml
COPY migrations  /app/migrations
COPY src/main.rs /app/src/main.rs
RUN cargo fetch
COPY src         /app/src
RUN cargo build --release
RUN cargo install --path .
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000
CMD ["alexandria"]
