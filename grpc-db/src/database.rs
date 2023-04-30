use proto::objects::{Goal, Guest, Image};
use scylla::{transport::session::Session, IntoTypedRows};
use std::error;
use tonic::Status;
use uuid::Uuid;

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

#[derive(Debug, scylla::FromRow, scylla::ValueList)]
pub struct GoalDB {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub progress_amt: i32,
    pub total_amt: i32,
    pub image: Uuid,
}

impl From<GoalDB> for Goal {
    fn from(goal_db: GoalDB) -> Self {
        Self {
            id: goal_db.id.to_string(),
            name: goal_db.name,
            description: goal_db.description,
            progress_amt: goal_db.progress_amt,
            total_amt: goal_db.total_amt,
            image: goal_db.image.to_string(),
        }
    }
}

impl From<Goal> for GoalDB {
    fn from(goal: Goal) -> Self {
        GoalDB {
            id: goal.id.parse().unwrap(),
            name: goal.name,
            description: goal.description,
            progress_amt: goal.progress_amt,
            total_amt: goal.total_amt,
            image: goal.image.parse().unwrap(),
        }
    }
}

pub async fn insert_goal(session: &Session, goal: Goal) -> Result<(), Status> {
    let goal_db: GoalDB = Into::<GoalDB>::into(goal);
    let resp = session
        .query(
            "INSERT INTO wedding.goal (id, name, description, progress_amt, total_amt, image) \
            VALUES (?, ?, ?, ?, ?, ?)",
            goal_db,
        )
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

pub async fn read_goal(session: &Session, id: String) -> Result<Goal, Status> {
    let resp = session
        .query(
            "SELECT id, name, description, progress_amt, total_amt, image FROM wedding.goal \
            WHERE id = ?",
            (to_uuid(id.as_str())?,),
        )
        .await;
    match resp {
        Ok(resp) => match resp.single_row_typed::<GoalDB>() {
            Ok(goal_db) => Ok(goal_db.into()),
            Err(e) => Err(Status::internal(format!("Failed to read from DB: {e}"))),
        },
        Err(e) => Err(Status::internal(format!("Failed to read from DB: {e}"))),
    }
}

pub async fn read_multi_goals(session: &Session) -> Result<Vec<Goal>, Status> {
    let resp = session
        .query(
            "SELECT id, name, description, progress_amt, total_amt, image FROM wedding.goal",
            &[],
        )
        .await;
    match resp {
        Ok(resp) => {
            let mut goals = vec![];
            if let Some(rows) = resp.rows {
                for row in rows.into_typed::<GoalDB>() {
                    match row {
                        Ok(goal) => goals.push(goal.into()),
                        Err(e) => {
                            return Err(Status::internal(format!("Failed to convert from DB: {e}")))
                        }
                    };
                }
                Ok(goals)
            } else {
                Err(Status::internal("Failed to read from DB"))
            }
        }
        Err(e) => Err(Status::internal(format!(
            "Failed to read multi goals from DB: {e}"
        ))),
    }
}

pub async fn delete_goal(session: &Session, id: String) -> Result<(), Status> {
    let resp = session
        .query(
            "DELETE FROM wedding.goal WHERE id = ?",
            (to_uuid(id.as_str())?,),
        )
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

#[derive(scylla::FromRow, scylla::ValueList)]
struct GuestDB {
    id: Uuid,
    phone_number: String,
    first_name: String,
    last_name: String,
    address: String,
    email: String,
    food_preferences: String,
    song_requests: String,
}

impl From<GuestDB> for Guest {
    fn from(guest_db: GuestDB) -> Self {
        Guest {
            id: guest_db.id.to_string(),
            phone_number: guest_db.phone_number,
            first_name: guest_db.first_name,
            last_name: guest_db.last_name,
            address: guest_db.address,
            email: guest_db.email,
            food_preferences: guest_db.food_preferences,
            song_requests: guest_db.song_requests,
        }
    }
}

impl From<Guest> for GuestDB {
    fn from(guest: Guest) -> Self {
        GuestDB {
            id: guest.id.parse().unwrap(),
            phone_number: guest.phone_number,
            first_name: guest.first_name,
            last_name: guest.last_name,
            address: guest.address,
            email: guest.email,
            food_preferences: guest.food_preferences,
            song_requests: guest.song_requests,
        }
    }
}

pub async fn insert_guest(session: &Session, guest: Guest) -> Result<(), Status> {
    let guest_db: GuestDB = Into::<GuestDB>::into(guest);
    let resp = session
        .query("INSERT INTO wedding.guest (id, phone_number, first_name, last_name, address, email, food_preferences, song_requests) \
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)", guest_db)  
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

pub async fn read_guest(session: &Session, id: String) -> Result<Guest, Status> {
    let resp = session
        .query(
            "SELECT id, phone_number, first_name, last_name, address, email, food_preferences, song_requests FROM wedding.guest \
            WHERE id = ?", (to_uuid(id.as_str())?,),
        )
        .await;
    match resp {
        Ok(resp) => match resp.single_row_typed::<GuestDB>() {
            Ok(guest_db) => Ok(guest_db.into()),
            Err(e) => Err(Status::internal(format!("Failed to read from DB: {e}"))),
        },
        Err(e) => Err(Status::internal(format!("Failed to read from DB: {e}"))),
    }
}

pub async fn read_multi_guests(session: &Session) -> Result<Vec<Guest>, Status> {
    let resp = session
        .query(
            "SELECT id, phone_number, first_name, last_name, address, email, food_preferences, song_requests FROM wedding.guest", 
            &[],
        )
        .await;
    match resp {
        Ok(resp) => {
            let mut guests = vec![];
            if let Some(rows) = resp.rows {
                for row in rows.into_typed::<GuestDB>() {
                    match row {
                        Ok(guest) => guests.push(guest.into()),
                        Err(e) => {
                            return Err(Status::internal(format!("Failed to convert from DB: {e}")))
                        }
                    };
                }
                Ok(guests)
            } else {
                Err(Status::internal("Failed to read from DB"))
            }
        }
        Err(e) => Err(Status::internal(format!(
            "Failed to read multi goals from DB: {e}"
        ))),
    }
}

pub async fn delete_guest(session: &Session, id: String) -> Result<(), Status> {
    let resp = session
        .query(
            "DELETE FROM wedding.guest WHERE id = ?",
            (to_uuid(id.as_str())?,),
        )
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

#[derive(scylla::FromRow, scylla::ValueList)]
struct ImageDB {
    id: Uuid,
    content: Vec<u8>,
    filename: String,
    extension: String,
}

impl From<ImageDB> for Image {
    fn from(image_db: ImageDB) -> Self {
        Image {
            id: image_db.id.to_string(),
            content: image_db.content,
            filename: image_db.filename,
            extension: image_db.extension,
        }
    }
}

impl From<Image> for ImageDB {
    fn from(image: Image) -> Self {
        ImageDB {
            id: image.id.parse().unwrap(),
            content: image.content,
            filename: image.filename,
            extension: image.extension,
        }
    }
}

pub async fn insert_image(session: &Session, image: Image) -> Result<(), Status> {
    let image_db: ImageDB = Into::<ImageDB>::into(image);
    let resp = session
        .query(
            "INSERT INTO wedding.image (id, content, filename, extension) \
               VALUES (?, ?, ?, ?)",
            image_db,
        )
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

pub async fn read_image(session: &Session, id: String) -> Result<Image, Status> {
    let resp = session
        .query(
            "SELECT id, content, filename, extension FROM wedding.image \
            WHERE id = ?",
            (to_uuid(id.as_str())?,),
        )
        .await;
    match resp {
        Ok(resp) => match resp.single_row_typed::<ImageDB>() {
            Ok(image_db) => Ok(image_db.into()),
            Err(e) => Err(Status::internal(format!("Failed to read from DB: {e}"))),
        },
        Err(e) => Err(Status::internal(format!("Failed to read from DB: {e}"))),
    }
}

pub async fn delete_image(session: &Session, id: String) -> Result<(), Status> {
    let resp = session
        .query(
            "DELETE FROM wedding.image WHERE id = ?",
            (to_uuid(id.as_str())?,),
        )
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

fn to_uuid(input: &str) -> Result<Uuid, Status> {
    match Uuid::parse_str(input) {
        Ok(uuid) => Ok(uuid),
        Err(e) => Err(Status::internal(format!(
            "Failed to parse str to uuid: {e}"
        ))),
    }
}
