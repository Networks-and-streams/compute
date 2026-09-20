FROM rust:1.78-slim AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
COPY proto ./proto
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /app/target/release/compute /usr/local/bin/compute
ENV COMPUTE_ADDR=0.0.0.0:50051
EXPOSE 50051
CMD ["compute"]
