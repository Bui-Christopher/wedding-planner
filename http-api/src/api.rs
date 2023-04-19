use poem_openapi::{
    param::{Path, Query},
    payload::{Json, PlainText},
    OpenApi,
};
use proto::wedding::{Goal, Guest, Image};

pub struct Api {}

impl Api {
    pub fn new() -> Self {
        Api {}
    }
}

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        debug!("Hello World!");
        match name.0 {
            Some(name) => PlainText(format!("hello, {name}!")),
            None => PlainText("hello!".to_string()),
        }
    }

    // API for Guests
    #[oai(path = "/guests", method = "post")]
    async fn create_guest(&self, guest: Json<Guest>) -> PlainText<String> {
        let username = &guest.username;
        debug!("Creating guest: {username}.");
        PlainText("Hello World".to_string())
    }

    #[oai(path = "/guests/:id", method = "get")]
    async fn read_guest(&self, id: Path<String>) -> PlainText<String> {
        let id = id.0;
        debug!("Reading registered guest: {id}.");
        PlainText("Hello World".to_string())
    }

    #[oai(path = "/guests", method = "get")]
    async fn read_guests(&self) -> PlainText<String> {
        debug!("Reading all registered guests...");
        PlainText("Hello World".to_string())
    }

    #[oai(path = "/guests", method = "patch")]
    async fn update_guest(&self, username: String) -> PlainText<String> {
        debug!("Updating guest: {username}.");
        PlainText("Hello World".to_string())
    }

    #[oai(path = "/guests/:id", method = "delete")]
    async fn delete_guest(&self, id: Path<String>) -> PlainText<String> {
        let id = id.0;
        debug!("Deleting guest: {id}.");
        PlainText("Hello World".to_string())
    }

    // API for Goals
    #[oai(path = "/goals", method = "post")]
    async fn create_goal(&self, goal: Json<Goal>) -> PlainText<String> {
        let name = &goal.name;
        debug!("Creating goal: {name}.");
        PlainText("Hello World".to_string())
    }

    #[oai(path = "/goals/:id", method = "get")]
    async fn read_goal(&self, id: Path<String>) -> PlainText<String> {
        let id = id.0;
        debug!("Reading specific goal: {id}.");
        PlainText("Hello World".to_string())
    }

    #[oai(path = "/goals", method = "get")]
    async fn read_gaols(&self) -> PlainText<String> {
        debug!("Reading all goals...");
        PlainText("Hello World".to_string())
    }

    #[oai(path = "/goals", method = "patch")]
    async fn update_goal(&self, name: String) -> PlainText<String> {
        debug!("Updating goal: {name}.");
        PlainText("Hello World".to_string())
    }

    #[oai(path = "/goals/:id", method = "delete")]
    async fn delete_goal(&self, id: Path<String>) -> PlainText<String> {
        let id = id.0;
        debug!("Deleting goal: {id}.");
        PlainText("Hello World".to_string())
    }

    // API for Images 
    #[oai(path = "/images", method = "post")]
    async fn create_image(&self, image: Json<Image>) -> PlainText<String> {
        let name= &image.name;
        debug!("Creating image: {name}.");
        PlainText("Hello World".to_string())
    }

    #[oai(path = "/images/:id", method = "get")]
    async fn read_image(&self, id: Path<String>) -> PlainText<String> {
        let id = id.0;
        debug!("Reading specific image: {id}.");
        PlainText("Hello World".to_string())
    }

    #[oai(path = "/images/:id", method = "delete")]
    async fn delete_image(&self, id: Path<String>) -> PlainText<String> {
        let id = id.0;
        debug!("Deleting image: {id}.");
        PlainText("Hello World".to_string())
    }
}
