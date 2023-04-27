#![deny(clippy::all)]
#[macro_use]
extern crate log;

mod goals;
mod guests;
mod images;

use crate::{goals::GoalService, guests::GuestService, images::ImageService};
use lazy_static::lazy_static;
use proto::methods::{
    goals_server::GoalsServer, guests_server::GuestsServer, images_server::ImagesServer,
};
use std::env::var;
use tonic::transport::Server;

lazy_static! {
    static ref MIDDLEWARE_URI: String = var("MIDDLEWARE_URI").unwrap_or("0.0.0.0:8081".to_string());
    static ref API_URI: String = var("API_URI").unwrap_or("0.0.0.0:8080".to_string());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let goals_service = GoalService::default();
    let images_service = ImageService::default();
    let guests_service = GuestService::default();

    info!("Starting grpc server");
    Server::builder()
        .add_service(GoalsServer::new(goals_service))
        .add_service(ImagesServer::new(images_service))
        .add_service(GuestsServer::new(guests_service))
        .serve("{MIDDLEWARE_URI}".parse()?)
        .await?;

    Ok(())
}
