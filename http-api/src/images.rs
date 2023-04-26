use proto::{
    methods::{images_client::ImagesClient, *},
    objects::Image,
};
use tonic::{transport::Channel, Request};
use uuid::Uuid;

pub async fn create_image(
    image: Image,
    channel: Channel,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let mut client = ImagesClient::new(channel);
    let req = Request::new(CreateImageRequest { image: Some(image) });

    let resp = client.create_image(req).await?;
    let resp = resp.into_inner();

    match Uuid::parse_str(&resp.id) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn read_image(id: String, channel: Channel) -> Result<Image, Box<dyn std::error::Error>> {
    let mut client = ImagesClient::new(channel);
    let req = Request::new(ReadImageRequest { id });

    let resp = client.read_image(req).await?;
    let resp = resp.into_inner();
    let image = resp.image;

    if let Some(image) = image {
        Ok(image)
    } else {
        Err("Failed to read image.".into())
    }
}

pub async fn delete_image(
    id: String,
    channel: Channel,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let mut client = ImagesClient::new(channel);
    let req = Request::new(DeleteImageRequest { id });

    let resp = client.delete_image(req).await?;
    let resp = resp.into_inner();

    match Uuid::parse_str(&resp.id) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Box::new(e)),
    }
}
