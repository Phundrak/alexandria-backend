FROM rust:1.63

WORKDIR /usr/src/alexandria
COPY . .

RUN cargo install --path .

CMD ["server"]
