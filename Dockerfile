FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo install cargo-watch

CMD [ "cargo", "watch", "-x", "run" ]
