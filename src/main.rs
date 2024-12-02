mod db;
mod auth;
mod server;
mod routes;
mod models;
mod controllers;

use std::sync::Arc;

use db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Establishes the connection to the database
    let client = establish_connection().await.expect("Failed to connect to database");
    
    // Wraps the database client in an Arc (atomic reference counted) for shared access across threads
    let client = Arc::new(client);

    // Starts the server, passing the database client as application data
    server::run_server(client).await
}





