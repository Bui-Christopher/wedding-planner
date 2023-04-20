#![deny(clippy::all)]

pub mod common;

pub mod wedding {
    tonic::include_proto!("wedding");
}
