use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio_postgres::Client;

use crate::routes::user_routes;


// Health check endpoint to verify if the server is running
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}

// Function to configure the routes for the application
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Configures the health check route and user routes
    cfg.route("/", web::get().to(health_check))
        .configure(user_routes::configure); // Configures the user-related routes
}

// Function to start the server and bind it to a host and port
pub async fn run_server(client: Arc<Client>) -> std::io::Result<()> {
    // Loads environment variables from the .env file
    dotenv().ok();

    // Retrieves the host and port from environment variables, or defaults to localhost and port 8080
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    // Prints the server starting information
    println!("Starting server at http://{}:{}", host, port);

    // Creates and runs the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone())) // Passes the database client as application data
            .configure(configure_routes) // Configures the routes (health check and user routes)
    })
    .bind((host.as_str(), port.parse::<u16>().unwrap()))? // Binds the server to the specified host and port
    .run() // Runs the server
    .await
}







