FROM rust:1.71.0

RUN mkdir -p /build
ENV CARGO_TARGET_DIR=/build

WORKDIR /app
COPY ./api .

RUN cargo build --release
ENTRYPOINT ["/build/release/nereus-ui"]
