# Wedding RSVP-Registry
A simple backend service for a Wedding RSVP and Registry. It is implemented with an http-api-gw, 
grpc-middleware, scylla-db, and a nginx proxy.

## HTTP-API
The HTTP API server utilizes the `poem-api crate`, thus, it creates its own swagger docs.

## gRPC-DB
The grpc middleware is the bridge between frontend HTTP and backend DB.

## PROTO
This is the library crate which defines the objects/methods that are passed between services.

# TODOS
There are several `todos` left to complete this project. 
- Proto
    - Confirm what db is being used to know if image.content should be sent as hex blob
- HTTP-API
    - Logs
- gRPC-DB
    - Write data to DB
    - Logs
- DB
    - Scylla
- NGINX
    - Set up self signed certs 
- Documentation
    - Improvements/TODOs
    - Setup/Quickstart

## Inspiration
Optimized Docker images
- https://blog.gheo.tech/posts/rust-docker-images/


`export RUST_LOG=debug`

# Inspiration
API responses inspired through poem/examples: 
- https://github.com/poem-web/poem/blob/master/examples/openapi/uniform-response/src/main.rs
- https://github.com/poem-web/poem/blob/master/examples/openapi/users-crud/src/main.rs
