# Wedding RSVP-Registry
A simple backend service for a Wedding RSVP and Registry. It implements an http-api-server and a grpc-db-server.

## HTTP-API
The HTTP API server utilizes the `poem-api crate`, thus, it creates its own swagger docs.

## gRPC-DB
This server is the bridge between frontend HTTP and backend DB.

## PROTO
This is the source of truth for both services.

# TODOS
There are several `todos` left to create a minimal viable product.
- Dockerize
    - Database (Cassandra/Redis?)
    - Build Image Scripts for both binaries
- Allow for optional fields
- And more! :(
