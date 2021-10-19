# This is a Dockerfile to build a rust image for development
FROM rust:1.54.0-slim-bullseye

RUN rustup component add clippy && rustup component add rustfmt
