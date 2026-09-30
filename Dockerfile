FROM rust:1.98-slim-bookworm AS build
WORKDIR /app
RUN apt-get update && apt-get install -y protobuf-compiler && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml Cargo.lock* build.rs ./
COPY src ./src
COPY proto ./proto
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /app/target/release/compute /usr/local/bin/compute
ENV COMPUTE_ADDR=0.0.0.0:50051
EXPOSE 50051
CMD ["compute"]
