FROM rust:1.63.0-slim-buster as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:buster-20220912-slim

WORKDIR /app

COPY --from=builder /app/target/release/protohackers .

CMD ["./protohackers"]
