FROM ubuntu:18.04

RUN apt update && apt install -y build-essential curl

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile default

ENV PATH="${PATH}:/root/.cargo/bin"

CMD cargo run
