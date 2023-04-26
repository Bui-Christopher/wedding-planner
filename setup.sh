cargo build --release --target=x86_64-unknown-linux-musl
mv target/x86_64-unknown-linux-musl/release/http-api .tmp/http-api
mv target/x86_64-unknown-linux-musl/release/grpc-db .tmp/grpc-db

docker build \
    -t http-api:latest \
    -f http-api/Dockerfile .tmp/ 

docker build \
    -t grpc-db:latest \
    -f grpc-db/Dockerfile .tmp/ 
