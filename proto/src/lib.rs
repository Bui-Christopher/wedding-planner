#![deny(clippy::all)]

pub mod common;

pub mod wedding {
    tonic::include_proto!("wedding");
}

pub mod service {
    tonic::include_proto!("service");
}
