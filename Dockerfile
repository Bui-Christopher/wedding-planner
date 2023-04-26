# FROM rust:1.69 AS builder
# RUN apt-get update && apt-get install -y protobuf-compiler
#
# WORKDIR /workspace
# COPY . .
# RUN cargo build --release
# # WORKDIR /app
# # COPY . .
# #
# # WORKDIR /app/proto
# # RUN cargo build --release
# #
# # WORKDIR /app/http-api
# # RUN cargo build --release
# #
# # WORKDIR /app/grpc-db
# # RUN cargo build --release
#
# FROM debian:buster-slim
# COPY --from=builder /workspace/http-api/target/release/http-api /usr/local/bin/http-api
#
# CMD ["/usr/local/bin/http-api"]
from scratch
copy http-api /http-api
cmd ["/http-api"]
