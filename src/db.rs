use tokio_postgres::{NoTls, Client};
use dotenv::dotenv;
use std::env;
use std::error::Error as StdError;

// Function to establish a connection to the database
pub async fn establish_connection() -> Result<Client, Box<dyn StdError>> {
    // Loads environment variables from the .env file
    dotenv().ok();

    // Retrieves the database URL from environment variables
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL not configured in .env")?;

    // Attempts to connect to the database
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .map_err(|e| format!("Error connecting to the database: {}", e))?;

    // Spawns a new async task to handle the connection
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            // Logs an error if the database connection fails
            eprintln!("Error with database connection: {}", e);
        }
    });

    // Returns the database client to interact with the database
    Ok(client)
}


