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
There are several `todos` left to complete this demo project. 
- HTTP-API
    - Dockerize
- gRPC-DB
    - Dockerize
    - Write data to DB
- DB
    - Cassandra/Scylla
- NGINX
    - Set up reverse proxy
Documentation
    - Improvements
    - Setup/Quickstart
