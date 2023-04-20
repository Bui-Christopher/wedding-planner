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
    async fn create_guest(&self, guest: Json<Guest>) -> Json<String> {
        let id = &guest.id;
        debug!("Creating guest: {id}.");
        Json("Creatin guest...".to_string())
    }

    #[oai(path = "/guests/:id", method = "get")]
    async fn read_guest(&self, id: Path<String>) -> Json<Guest> {
        let id = id.0;
        debug!("Reading registered guest: {id}.");
        Json(Guest::default())
    }

    #[oai(path = "/guests", method = "get")]
    async fn read_guests(&self) -> Json<Vec<Guest>> {
        debug!("Reading all registered guests...");
        Json(vec![])
    }

    #[oai(path = "/guests/:id", method = "patch")]
    async fn update_guest(&self, id: Path<String>) -> Json<String> {
        let id = id.0;
        debug!("Updating guest: {id}.");
        Json("Updating guest...".to_string())
    }

    #[oai(path = "/guests/:id", method = "delete")]
    async fn delete_guest(&self, id: Path<String>) -> Json<String> {
        let id = id.0;
        debug!("Deleting guest: {id}.");
        Json("Deleting guest...".to_string())
    }

    // API for Goals
    #[oai(path = "/goals", method = "post")]
    async fn create_goal(&self, goal: Json<Goal>) -> Json<String> {
        let id = &goal.id;
        debug!("Creating goal: {id}.");
        Json("Creating goal...".to_string())
    }

    #[oai(path = "/goals/:id", method = "get")]
    async fn read_goal(&self, id: Path<String>) -> Json<Goal> {
        let id = id.0;
        debug!("Reading specific goal: {id}.");
        Json(Goal::default())
    }

    #[oai(path = "/goals", method = "get")]
    async fn read_goals(&self) -> Json<Vec<Goal>> {
        debug!("Reading all goals...");
        Json(vec![])
    }

    #[oai(path = "/goals/:id", method = "patch")]
    async fn update_goal(&self, id: Path<String>) -> Json<String> {
        let id = id.0;
        debug!("Updating goal: {id}.");
        Json("Updating goal...".to_string())
    }

    #[oai(path = "/goals/:id", method = "delete")]
    async fn delete_goal(&self, id: Path<String>) -> Json<String> {
        let id = id.0;
        debug!("Deleting goal: {id}.");
        Json("Deleting goal...".to_string())
    }

    // API for Images
    // TODO: Submit binary
    #[oai(path = "/images", method = "post")]
    async fn create_image(&self, image: Json<Image>) -> Json<String> {
        let id = &image.id;
        debug!("Creating image: {id}.");
        Json("Creating image...".to_string())
    }

    // TODO: Return Bunary
    #[oai(path = "/images/:id", method = "get")]
    async fn read_image(&self, id: Path<String>) -> Json<Image> {
        let id = id.0;
        debug!("Reading specific image: {id}.");
        Json(Image::default())
    }

    #[oai(path = "/images/:id", method = "delete")]
    async fn delete_image(&self, id: Path<String>) -> Json<String> {
        let id = id.0;
        debug!("Deleting image: {id}.");
        Json("Deleting image...".to_string())
    }
}
