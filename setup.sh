#!/bin/bash
mkdir -p .tmp

cargo build --release --target=x86_64-unknown-linux-musl
mv target/x86_64-unknown-linux-musl/release/http-api .tmp/http-api
mv target/x86_64-unknown-linux-musl/release/grpc-db .tmp/grpc-db

docker build \
    -t cbui125/http-api:$(git describe) \
    -f http-api/Dockerfile .tmp/ 
docker push cbui125/http-api:$(git describe)

docker build \
    -t cbui125/grpc-db:$(git describe) \
    -f grpc-db/Dockerfile .tmp/ 
docker push cbui125/grpc-db:$(git describe)
