#![deny(clippy::all)]
#[macro_use]
extern crate log;

mod api;
mod goals;
mod guests;
mod images;
mod responses;

use lazy_static::lazy_static;
use poem::{listener::TcpListener, Route};
use poem_openapi::OpenApiService;
use std::env::var;

lazy_static! {
    static ref MIDDLEWARE_URI: String = var("MIDDLEWARE_URI").unwrap_or("http://localhost:8081".to_string());
    static ref API_URI: String = var("API_URI").unwrap_or("0.0.0.0:8080".to_string());
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();
    let channel = tonic::transport::Channel::from_static(&MIDDLEWARE_URI).connect_lazy();
    let api = api::Api::new(channel);
    let api_service =
        OpenApiService::new(api, "RSVP/Registry API", "1.0");

    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api/v1/", api_service).nest("/api/v1/doc", ui);

    info!("Starting http server");
    poem::Server::new(TcpListener::bind(API_URI.to_string()))
        .run(app)
        .await
}
