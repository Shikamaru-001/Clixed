FROM rust:1.88

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release
EXPOSE 3001

CMD ["./target/release/clixed"]
