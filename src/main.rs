mod jwt;
mod controllers;
mod db;
mod models;
mod routes;
mod server;
mod auth;

use std::sync::Arc;

use db::establish_connection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
    // Establishes the connection to the database
    let client = establish_connection()
        .await
        .expect("Failed to connect to database");

    // Wraps the database client in an Arc (atomic reference counted) for shared access across threads
    let client = Arc::new(client);

    // Starts the server, passing the database client as application data
    server::run_server(client).await
}
