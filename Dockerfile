FROM rust:1.71.0

RUN mkdir -p /build
ENV CARGO_TARGET_DIR=/build

WORKDIR /app
# COPY ./api .

RUN cargo install cargo-watch 

# EXPOSE 8080
ENTRYPOINT cargo watch -- cargo run
