#![deny(clippy::all)]
#[macro_use]
extern crate log;

mod api;
mod goals;
mod guests;
mod images;

use poem::{listener::TcpListener, Route};
use poem_openapi::OpenApiService;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let api = api::Api::new();
    let api_service =
        OpenApiService::new(api, "Wedding API", "1.0").server("http://localhost:8080/api/v1");

    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api/v1", api_service).nest("/", ui);

    info!("Starting http server");
    poem::Server::new(TcpListener::bind("0.0.0.0:8080"))
        .run(app)
        .await
}
