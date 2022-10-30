FROM rust:1.64

WORKDIR /usr/src/alexandria
COPY . .

RUN cargo install --path .

CMD ["alexandria"]
