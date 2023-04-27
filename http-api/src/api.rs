use crate::{
    goals::*,
    guests::*,
    images::*,
    responses::{BinaryResponse, JsonResponse},
};

use poem_openapi::{
    param::{Path, Query},
    payload::{Binary, Json, PlainText, Response},
    OpenApi,
};
use proto::objects::{Goal, Guest, Image};
use tonic::transport::Channel;
use uuid::Uuid;

pub struct Api {
    grpc_channel: Channel,
}

impl Api {
    pub fn new(channel: Channel) -> Self {
        Api {
            grpc_channel: channel,
        }
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

    // Handler for Guests
    #[oai(path = "/guests", method = "post")]
    async fn create_guest(&self, guest: Json<Guest>) -> JsonResponse<Uuid> {
        let guest = guest.0;
        debug!("Creating guest: {:?}.", guest);
        match create_guest(guest, self.grpc_channel.clone()).await {
            Ok(uuid) => JsonResponse::ok(uuid),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }

    #[oai(path = "/guests/:id", method = "get")]
    async fn read_guest(&self, id: Path<Uuid>) -> JsonResponse<Guest> {
        let id = id.0.to_string();
        debug!("Reading registered guest: {id}.");
        match read_guest(id, self.grpc_channel.clone()).await {
            Ok(guest) => JsonResponse::ok(guest),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }

    #[oai(path = "/guests", method = "get")]
    async fn read_guests(&self) -> JsonResponse<Vec<Guest>> {
        debug!("Reading all registered guests...");
        match read_multi_guests(self.grpc_channel.clone()).await {
            Ok(guests) => JsonResponse::ok(guests),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }

    #[oai(path = "/guests/", method = "patch")]
    async fn update_guest(&self, guest: Json<Guest>) -> JsonResponse<Uuid> {
        let guest = guest.0;
        debug!("Updating guest: {:?}.", guest);
        match update_guest(guest, self.grpc_channel.clone()).await {
            Ok(uuid) => JsonResponse::ok(uuid),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }

    #[oai(path = "/guests/:id", method = "delete")]
    async fn delete_guest(&self, id: Path<Uuid>) -> JsonResponse<Uuid> {
        let id = id.0.to_string();
        debug!("Deleting guest: {id}.");
        match delete_guest(id, self.grpc_channel.clone()).await {
            Ok(uuid) => JsonResponse::ok(uuid),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }

    // Handler or Goals
    #[oai(path = "/goals", method = "post")]
    async fn create_goal(&self, goal: Json<Goal>) -> JsonResponse<Uuid> {
        let goal = goal.0;
        debug!("Creating goal: {:?}.", goal);
        match create_goal(goal, self.grpc_channel.clone()).await {
            Ok(uuid) => JsonResponse::ok(uuid),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }

    #[oai(path = "/goals/:id", method = "get")]
    async fn read_goal(&self, id: Path<Uuid>) -> JsonResponse<Goal> {
        let id = id.0.to_string();
        debug!("Reading specific goal: {id}.");
        match read_goal(id, self.grpc_channel.clone()).await {
            Ok(goal) => JsonResponse::ok(goal),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }

    #[oai(path = "/goals", method = "get")]
    async fn read_goals(&self) -> JsonResponse<Vec<Goal>> {
        debug!("Reading all goals...");
        match read_multi_goals(self.grpc_channel.clone()).await {
            Ok(goals) => JsonResponse::ok(goals),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }

    #[oai(path = "/goals", method = "patch")]
    async fn update_goal(&self, goal: Json<Goal>) -> JsonResponse<Uuid> {
        let goal = goal.0;
        debug!("Updating goal: {:?}.", goal);
        match update_goal(goal, self.grpc_channel.clone()).await {
            Ok(uuid) => JsonResponse::ok(uuid),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }

    #[oai(path = "/goals/:id", method = "delete")]
    async fn delete_goal(&self, id: Path<Uuid>) -> JsonResponse<Uuid> {
        let id = id.0.to_string();
        debug!("Deleting goal: {id}.");
        match delete_goal(id, self.grpc_channel.clone()).await {
            Ok(uuid) => JsonResponse::ok(uuid),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }

    // Handler or Images
    #[oai(path = "/images", method = "post")]
    async fn create_image(
        &self,
        content: Binary<Vec<u8>>,
        filename: Query<String>,
        extension: Query<String>,
    ) -> JsonResponse<Uuid> {
        let content = content.0;
        let filename = filename.0.to_string();
        let extension = extension.0.to_string();
        debug!("Creating image...");
        let image = Image {
            content,
            filename,
            extension,
            id: Uuid::new_v4().to_string(),
        };
        match create_image(image, self.grpc_channel.clone()).await {
            Ok(uuid) => JsonResponse::ok(uuid),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }

    #[oai(path = "/images/:id", method = "get")]
    async fn read_image(&self, id: Path<Uuid>) -> Response<BinaryResponse> {
        let id = id.0.to_string();
        debug!("Reading specific image: {id}.");
        match read_image(id, self.grpc_channel.clone()).await {
            Ok(image) => BinaryResponse::ok(Binary(image.content), image.filename, image.extension),
            Err(error) => BinaryResponse::internal_error(error.to_string()).into(),
        }
    }

    #[oai(path = "/images/:id", method = "delete")]
    async fn delete_image(&self, id: Path<Uuid>) -> JsonResponse<Uuid> {
        let id = id.0.to_string();
        debug!("Deleting image: {id}.");
        match delete_image(id, self.grpc_channel.clone()).await {
            Ok(uuid) => JsonResponse::ok(uuid),
            Err(error) => JsonResponse::internal_error(error.to_string()),
        }
    }
}
