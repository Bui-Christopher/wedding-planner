# Wedding RSVP-Registry
A simple backend service for a Wedding RSVP and Registry. It is implemented with an http-api-gw, 
grpc-middleware, scylla-db, and an nginx proxy.

## HTTP-API
The HTTP API server utilizes the `poem-api crate`, thus, it creates its own swagger docs.
The endpoint is hosted on `localhost/api/v1` and the swagger page is hosted on `localhost/api/v1/doc`

## gRPC-DB
The grpc middleware is the bridge between frontend HTTP and backend DB. It is a gRPC server and is flexible
enough to be written in any language using the `objects.proto` and `methods.proto`.

## PROTO
This is the library crate which defines the objects/methods that are passed between services. This includes
goals, guests, and images.

## Scylla
Scylla stores the data locally in the './db' folder. To restart local data, `docker compose kill && docker compose down`
then delete the `./db` folder. 

It's also possible to manipulate the data from within the container. `docker exec -it scylla cqlsh`
Some sample commands include:
- `select * from wedding.image;`
- `select * from wedding.guest;`
- `truncate wedding.guest` Deletes all guest elements from the db

## Setup 
```
cd wedding-planner
docker compose up -d
docker compose kill && docker compose down --remove-orphans
```

## Docker
Logs are set through env variables: `export RUST_LOG=debug`
`setup.sh` Used to compile/build docker images

## Inspiration
- Optimized Docker images
    - https://blog.gheo.tech/posts/rust-docker-images/

- API responses inspired through poem/examples: 
    - https://github.com/poem-web/poem/blob/master/examples/openapi/uniform-response/src/main.rs
    - https://github.com/poem-web/poem/blob/master/examples/openapi/users-crud/src/main.rs

## Improvements
- `database.rs`
    - Complete refactor
    - Flexibility
        - Currently CRUD queries/operations are hard coded
- Authentication
    - Can start with something simple like basic authentication
- Automation scripts
- Additional HTTP codes
- Error handling is loosely done
- github workflows

## TODOS
There are several `todos` left to complete this project. 
- HTTP-API
    - Logs
- gRPC-DB
    - Write data to DB
    - Logs
- DB
    - Connect to managed service (possibly use cql_proxy)
- NGINX
    - Set up self signed certs 
- Docker
    - Push images to dockerhub
- Documentation
    - Improvements
    - Setup/Quickstart
