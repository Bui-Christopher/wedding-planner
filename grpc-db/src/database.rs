use scylla::transport::session::Session;
use std::error;

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
