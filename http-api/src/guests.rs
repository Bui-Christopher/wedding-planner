use proto::{
    methods::{guests_client::GuestsClient, *},
    objects::Guest,
};
use tonic::{transport::Channel, Request};
use uuid::Uuid;

pub async fn create_guest(
    guest: Guest,
    channel: Channel,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let mut client = GuestsClient::new(channel);
    let req = Request::new(CreateGuestRequest { guest: Some(guest) });

    let resp = client.create_guest(req).await?;
    let resp = resp.into_inner();

    match Uuid::parse_str(&resp.id) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn read_guest(id: String, channel: Channel) -> Result<Guest, Box<dyn std::error::Error>> {
    let mut client = GuestsClient::new(channel);
    let req = Request::new(ReadGuestRequest { id });

    let resp = client.read_guest(req).await?;
    let resp = resp.into_inner();
    let guest = resp.guest;

    if let Some(guest) = guest {
        Ok(guest)
    } else {
        Err("Failed to read guest.".into())
    }
}

pub async fn read_multi_guests(channel: Channel) -> Result<Vec<Guest>, Box<dyn std::error::Error>> {
    let mut client = GuestsClient::new(channel);
    let req = Request::new(ReadMultiGuestsRequest {});

    let resp = client.read_multi_guests(req).await?;
    let resp = resp.into_inner();

    Ok(resp.guests)
}

pub async fn update_guest(
    guest: Guest,
    channel: Channel,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let mut client = GuestsClient::new(channel);
    let req = Request::new(UpdateGuestRequest { guest: Some(guest) });

    let resp = client.update_guest(req).await?;
    let resp = resp.into_inner();

    match Uuid::parse_str(&resp.id) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Box::new(e)),
    }
}

pub async fn delete_guest(
    id: String,
    channel: Channel,
) -> Result<Uuid, Box<dyn std::error::Error>> {
    let mut client = GuestsClient::new(channel);
    let req = Request::new(DeleteGuestRequest { id });

    let resp = client.delete_guest(req).await?;
    let resp = resp.into_inner();

    match Uuid::parse_str(&resp.id) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Box::new(e)),
    }
}
