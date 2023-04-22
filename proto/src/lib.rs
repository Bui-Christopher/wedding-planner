#![deny(clippy::all)]

pub mod common;

pub mod objects {
    tonic::include_proto!("objects");
}

pub mod methods {
    tonic::include_proto!("methods");
}
