FROM rust:1.63.0-slim-buster as builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:buster-20220912-slim

WORKDIR /app

ADD https://github.com/DarthSim/hivemind/releases/download/v1.1.0/hivemind-v1.1.0-linux-amd64.gz ./hivemind.gz
RUN gunzip hivemind.gz && chmod +x hivemind

COPY Procfile .
COPY --from=builder /app/target/release/protohackers .

CMD ["./hivemind"]
