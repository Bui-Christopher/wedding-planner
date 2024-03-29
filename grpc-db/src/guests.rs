use proto::{
    methods::{guests_server::Guests, *},
    objects::Guest,
};
use scylla::transport::session::Session;
use tonic::{Request, Response, Status};

use crate::database;

pub struct GuestService {
    db_session: Session,
}

impl GuestService {
    pub(super) fn new(db_session: Session) -> Self {
        Self { db_session }
    }
}

pub fn init_guest_server(db_session: Session) -> guests_server::GuestsServer<GuestService> {
    guests_server::GuestsServer::new(GuestService::new(db_session))
}

#[tonic::async_trait]
impl Guests for GuestService {
    async fn create_guest(
        &self,
        req: Request<CreateGuestRequest>,
    ) -> Result<Response<CreateGuestResponse>, Status> {
        let CreateGuestRequest { guest } = req.into_inner();
        let guest = safely_extract(guest)?;
        let id = guest.id.clone();

        database::insert_guest(&self.db_session, guest).await?;
        let resp = CreateGuestResponse { id };
        Ok(Response::new(resp))
    }

    async fn read_guest(
        &self,
        req: Request<ReadGuestRequest>,
    ) -> Result<Response<ReadGuestResponse>, Status> {
        let ReadGuestRequest { id } = req.into_inner();
        let guest = database::read_guest(&self.db_session, id).await?;

        let resp = ReadGuestResponse { guest: Some(guest) };
        Ok(Response::new(resp))
    }

    async fn read_multi_guests(
        &self,
        req: Request<ReadMultiGuestsRequest>,
    ) -> Result<Response<ReadMultiGuestsResponse>, Status> {
        let ReadMultiGuestsRequest {} = req.into_inner();

        let guests = database::read_multi_guests(&self.db_session).await?;
        let resp = ReadMultiGuestsResponse { guests };
        Ok(Response::new(resp))
    }

    async fn update_guest(
        &self,
        req: Request<UpdateGuestRequest>,
    ) -> Result<Response<UpdateGuestResponse>, Status> {
        let UpdateGuestRequest { guest } = req.into_inner();
        let guest = safely_extract(guest)?;
        let id = guest.id.clone();

        database::insert_guest(&self.db_session, guest).await?;
        let resp = UpdateGuestResponse { id };
        Ok(Response::new(resp))
    }

    async fn delete_guest(
        &self,
        req: Request<DeleteGuestRequest>,
    ) -> Result<Response<DeleteGuestResponse>, Status> {
        let DeleteGuestRequest { id } = req.into_inner();

        database::delete_guest(&self.db_session, id.clone()).await?;
        let resp = DeleteGuestResponse { id };
        Ok(Response::new(resp))
    }
}

fn safely_extract(guest: Option<Guest>) -> Result<Guest, Status> {
    match guest {
        Some(guest) => Ok(guest),
        _ => Err(Status::invalid_argument("Not a valid Guest Object.")),
    }
}
