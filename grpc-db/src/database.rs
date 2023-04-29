use proto::objects::{Goal, Guest, Image};
use scylla::transport::session::Session;
use uuid::Uuid;
use std::error;
use tonic::Status;

pub async fn initalize_keyspace(session: &Session) -> Result<(), Box<dyn error::Error>> {
    session.query("CREATE KEYSPACE IF NOT EXISTS wedding WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}", &[]).await?;
    Ok(())
}

pub async fn initalize_tables(session: &Session) -> Result<(), Box<dyn error::Error>> {
    session
        .query(
            "CREATE TABLE IF NOT EXISTS wedding.goal (id uuid, name text, description text, progress_amt int, total_amt int, image uuid, PRIMARY KEY (id))",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE TABLE IF NOT EXISTS wedding.guest (id uuid, phone_number text, first_name text, last_name text, address text, email text, food_preferences text, song_requests text, PRIMARY KEY (id))",
            &[],
        )
        .await?;
    session
        .query(
            "CREATE TABLE IF NOT EXISTS wedding.image (id uuid, content blob, filename text, \
            extension text, PRIMARY KEY (id))",
            &[],
        )
        .await?;
    Ok(())
}

pub async fn create_goal(session: &Session, goal: &Goal) -> Result<(), Status> {
    debug!("{:?}",goal);
    let resp = session
        .query("INSERT INTO wedding.goal (id, name, description, progress_amt, total_amt, image) VALUES (?, ?, ?, ?, ?, ?)", (Uuid::parse_str(goal.id.as_str()).unwrap(), goal.name.clone(), goal.description.clone(), goal.progress_amt, goal.total_amt, Uuid::parse_str(goal.image.as_str()).unwrap()))
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}
pub async fn read_goal(session: &Session, goal: &Goal) -> Result<(), Status> {
    Ok(())
}
pub async fn read_multi_goals(session: &Session, goal: &Goal) -> Result<(), Status> {
    Ok(())
}
pub async fn update_goal(session: &Session, goal: &Goal) -> Result<(), Status> {
    let resp = session
        .query("INSERT INTO wedding.goal (id, name, description, progress_amt, total_amt, image) VALUES (?, ?, ?, ?, ?, ?)", goal)
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}
pub async fn delete_goal(session: &Session, id: String) -> Result<(), Status> {
    let resp = session
        .query("DELETE FROM wedding.goal WHERE id = (?)", (id,))
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

pub async fn create_guest(session: &Session, guest: &Guest) -> Result<(), Status> {
    debug!("{:?}", guest);
    let resp = session
        .query("INSERT INTO wedding.guest (id, phone_number, first_name, last_name, address, email, food_preferences, song_requests) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", guest)
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}
pub async fn read_guest(session: &Session, guest: &Guest) -> Result<(), Status> {
    Ok(())
}
pub async fn read_multi_guests(session: &Session, guest: &Guest) -> Result<(), Status> {
    Ok(())
}
pub async fn update_guest(session: &Session, guest: &Guest) -> Result<(), Status> {
    Ok(())
}
pub async fn delete_guest(session: &Session, id: String) -> Result<(), Status> {
    let resp = session
        .query("DELETE FROM wedding.guest WHERE id = (?)", (id,))
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

pub async fn create_image(session: &Session, image: &Image) -> Result<(), Status> {
    Ok(())
}
pub async fn read_image(session: &Session, image: &Image) -> Result<(), Status> {
    Ok(())
}
pub async fn delete_image(session: &Session, id: String) -> Result<(), Status> {
    let resp = session
        .query("DELETE FROM wedding.image WHERE id = (?)", (id,))
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}
