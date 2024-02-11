FROM rust:1.75.0 as development

WORKDIR /app
RUN cargo install cargo-watch

COPY . .

FROM development as builder
RUN cargo build --release
