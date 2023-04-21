#![deny(clippy::all)]
#[macro_use]
extern crate log;

mod api;
mod goals;
mod guests;
mod images;
mod responses;

use poem::{listener::TcpListener, Route};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let channel = tonic::transport::Channel::from_static("http://[::1]:50051").connect_lazy();
    let api = api::Api::new(channel);
    let api_service =
        OpenApiService::new(api, "RSVP/Registry API", "1.0").server("http://localhost:3000/api/v1");

    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api/v1", api_service).nest("/", ui);

    info!("Starting http server");
    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
