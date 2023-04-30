#![deny(clippy::all)]
#[macro_use]
extern crate log;

mod database;
mod goals;
mod guests;
mod images;

use crate::{goals::init_goal_server, guests::init_guest_server, images::init_image_server};
use lazy_static::lazy_static;
use scylla::transport::session::Session;
use scylla::SessionBuilder;
use std::env::{var, var_os};
use tonic::transport::Server;

lazy_static! {
    static ref MIDDLEWARE_URI: String = var("MIDDLEWARE_URI").unwrap_or("0.0.0.0:8081".to_string());
    static ref API_URI: String = var("API_URI").unwrap_or("0.0.0.0:8080".to_string());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let uri = var("SCYLLA_URI").unwrap_or_else(|_| "scylla:9042".to_string());
    let init_db_session: Session = SessionBuilder::new()
        .known_node(uri.clone())
        .build()
        .await?;
    let goal_db_session: Session = SessionBuilder::new()
        .known_node(uri.clone())
        .build()
        .await?;
    let image_db_session: Session = SessionBuilder::new()
        .known_node(uri.clone())
        .build()
        .await?;
    let guest_db_session: Session = SessionBuilder::new().known_node(uri).build().await?;

    match var_os("IS_LOCAL_DEV") {
        Some(_) => database::initalize_keyspace(&init_db_session).await?,
        None => {}
    }
    database::initalize_tables(&init_db_session).await?;
    env_logger::init();

    info!("Starting grpc server");
    Server::builder()
        .add_service(init_goal_server(goal_db_session))
        .add_service(init_guest_server(guest_db_session))
        .add_service(init_image_server(image_db_session))
        .serve("0.0.0.0:8081".parse()?)
        .await?;

    Ok(())
}
