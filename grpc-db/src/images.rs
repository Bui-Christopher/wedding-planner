use proto::{
    methods::{images_server::Images, *},
    objects::Image,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct ImageService {}

#[tonic::async_trait]
impl Images for ImageService {
    async fn create_image(
        &self,
        req: Request<CreateImageRequest>,
    ) -> Result<Response<CreateImageResponse>, Status> {
        let CreateImageRequest { image } = req.into_inner();
        let image = safely_extract(image)?;
        let resp = CreateImageResponse { id: image.id };
        Ok(Response::new(resp))
    }

    async fn read_image(
        &self,
        req: Request<ReadImageRequest>,
    ) -> Result<Response<ReadImageResponse>, Status> {
        let ReadImageRequest { id: _id } = req.into_inner();
        let image = Image::default();
        let resp = ReadImageResponse { image: Some(image) };
        Ok(Response::new(resp))
    }

    async fn delete_image(
        &self,
        req: Request<DeleteImageRequest>,
    ) -> Result<Response<DeleteImageResponse>, Status> {
        let DeleteImageRequest { id } = req.into_inner();
        let resp = DeleteImageResponse { id };
        Ok(Response::new(resp))
    }
}

fn safely_extract(image: Option<Image>) -> Result<Image, Status> {
    match image {
        Some(image) => Ok(image),
        _ => Err(Status::invalid_argument("Not a valid Image Object.")),
    }
}