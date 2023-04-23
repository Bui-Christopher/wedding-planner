use proto::{
    methods::{guests_server::Guests, *},
    objects::Guest,
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct GuestService {}

#[tonic::async_trait]
impl Guests for GuestService {
    async fn create_guest(
        &self,
        req: Request<CreateGuestRequest>,
    ) -> Result<Response<CreateGuestResponse>, Status> {
        let CreateGuestRequest { guest } = req.into_inner();
        let guest = safely_extract(guest)?;
        let resp = CreateGuestResponse { id: guest.id };
        Ok(Response::new(resp))
    }

    async fn read_guest(
        &self,
        req: Request<ReadGuestRequest>,
    ) -> Result<Response<ReadGuestResponse>, Status> {
        let ReadGuestRequest { id: _id } = req.into_inner();
        let guest = Guest::default();
        let resp = ReadGuestResponse { guest: Some(guest) };
        Ok(Response::new(resp))
    }

    async fn read_multi_guests(
        &self,
        req: Request<ReadMultiGuestsRequest>,
    ) -> Result<Response<ReadMultiGuestsResponse>, Status> {
        let ReadMultiGuestsRequest {} = req.into_inner();
        let guests = vec![];
        let resp = ReadMultiGuestsResponse { guests };
        Ok(Response::new(resp))
    }

    async fn update_guest(
        &self,
        req: Request<UpdateGuestRequest>,
    ) -> Result<Response<UpdateGuestResponse>, Status> {
        let UpdateGuestRequest { guest } = req.into_inner();
        let guest = safely_extract(guest)?;
        let resp = UpdateGuestResponse { id: guest.id };
        Ok(Response::new(resp))
    }

    async fn delete_guest(
        &self,
        req: Request<DeleteGuestRequest>,
    ) -> Result<Response<DeleteGuestResponse>, Status> {
        let DeleteGuestRequest { id } = req.into_inner();
        let resp = DeleteGuestResponse { id };
        Ok(Response::new(resp))
    }
}

fn safely_extract(guest: Option<Guest>) -> Result<Guest, Status> {
    match guest {
        Some(guest) => Ok(guest),
        _ => Err(Status::invalid_argument("Not a valid Guest Object."))
    }
}
