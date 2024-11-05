use tokio;
use tokio_postgres::{Error, NoTls};

pub async fn connect_and_setup() -> Result<(), Error> {
    // Connect to the database
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=emilt password=123456 dbname=vocapp",
        NoTls,
    )
    .await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create an example user schema
    client
        .batch_execute(
            "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL UNIQUE
        )
    ",
        )
        .await?;

    println!("Connected to the database and ensured user schema exists.");

    Ok(())
}
