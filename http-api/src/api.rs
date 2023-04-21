use poem_openapi::{
    param::{Path, Query},
    payload::{Binary, Json, PlainText},
    OpenApi,
};
use proto::wedding::{Goal, Guest, Image};
use uuid::Uuid;

pub struct Api {}

impl Api {
    pub fn new() -> Self {
        Api {}
    }
}

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn hello(&self, name: Query<Option<String>>) -> PlainText<String> {
        debug!("Hello World!");
        match name.0 {
            Some(name) => PlainText(format!("hello, {name}!")),
            None => PlainText("hello!".to_string()),
        }
    }

    // API for Guests
    #[oai(path = "/guests", method = "post")]
    async fn create_guest(&self, guest: Json<Guest>) -> Json<String> {
        let guest = guest.0;
        debug!("Creating guest: {:?}.", guest);
        Json(guest.id)
    }

    #[oai(path = "/guests/:id", method = "get")]
    async fn read_guest(&self, id: Path<Uuid>) -> Json<Guest> {
        let id = id.0.to_string();
        debug!("Reading registered guest: {id}.");
        Json(Guest::default())
    }

    #[oai(path = "/guests", method = "get")]
    async fn read_guests(&self) -> Json<Vec<Guest>> {
        debug!("Reading all registered guests...");
        Json(vec![])
    }

    #[oai(path = "/guests/", method = "patch")]
    async fn update_guest(&self, guest: Json<Guest>) -> Json<String> {
        let guest = guest.0; 
        debug!("Updating guest: {:?}.", guest);
        Json(guest.id)
    }

    #[oai(path = "/guests/:id", method = "delete")]
    async fn delete_guest(&self, id: Path<Uuid>) -> Json<String> {
        let id = id.0.to_string();
        debug!("Deleting guest: {id}.");
        Json(id)
    }

    // API for Goals
    #[oai(path = "/goals", method = "post")]
    async fn create_goal(&self, goal: Json<Goal>) -> Json<String> {
        let goal = goal.0;
        debug!("Creating goal: {:?}.", goal);
        Json(goal.id)
    }

    #[oai(path = "/goals/:id", method = "get")]
    async fn read_goal(&self, id: Path<Uuid>) -> Json<Goal> {
        let id = id.0.to_string();
        debug!("Reading specific goal: {id}.");
        Json(Goal::default())
    }

    #[oai(path = "/goals", method = "get")]
    async fn read_goals(&self) -> Json<Vec<Goal>> {
        debug!("Reading all goals...");
        Json(vec![])
    }

    #[oai(path = "/goals", method = "patch")]
    async fn update_goal(&self, goal: Json<Goal>) -> Json<String> {
        let goal = goal.0;
        debug!("Updating goal: {:?}.", goal);
        Json(goal.id)
    }

    #[oai(path = "/goals/:id", method = "delete")]
    async fn delete_goal(&self, id: Path<Uuid>) -> Json<String> {
        let id = id.0.to_string();
        debug!("Deleting goal: {id}.");
        Json(id)
    }

    // API for Images
    #[oai(path = "/images", method = "post")]
    async fn create_image(&self, content: Binary<Vec<u8>>) -> Json<String> {
        let _content = content.0; 
        debug!("Creating image...");
        Json(Uuid::new_v4().to_string())
    }

    #[oai(path = "/images/:id", method = "get")]
    async fn read_image(&self, id: Path<Uuid>) -> Binary<Vec<u8>> {
        let id = id.0.to_string();
        debug!("Reading specific image: {id}.");
        Binary(Image::default().content)
    }

    #[oai(path = "/images/:id", method = "delete")]
    async fn delete_image(&self, id: Path<Uuid>) -> Json<String> {
        let id = id.0.to_string();
        debug!("Deleting image: {id}.");
        Json(id)
    }
}
