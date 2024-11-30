use std::sync::Arc;

use actix_web::{web, HttpResponse};
use tokio_postgres::Client;
use uuid::Uuid;
use serde::Deserialize;

use crate::models::user::User;

/// Struct representing the request data for creating a user.
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,       // User's name
    pub email: String,      // User's email
    pub password: String,   // User's password (to be hashed)
}

/// Handler function to create a user.
/// This function processes the incoming HTTP request, hashes the password, and
/// stores the user in the database.
pub async fn create_user(
    client: web::Data<Arc<Client>>,
    user_data: web::Json<CreateUserRequest>,
) -> HttpResponse {
    // Call the create_user method from the User model to insert the new user into the database
    let user = User::create_user(&client, &user_data.name, &user_data.email, &user_data.password).await;

    match user {
        // If user creation is successful, return the user data as JSON
        Ok(user) => HttpResponse::Ok().json(user),
        // If there's an error during creation, return an internal server error response
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Handler function to fetch a user by their ID.
/// This function queries the database to retrieve the user based on the given ID.
pub async fn get_user(client: web::Data<Arc<Client>>, user_id: web::Path<Uuid>) -> HttpResponse {
    // Call the get_user method from the User model to fetch the user data
    let user = User::get_user(&client, user_id.into_inner()).await;

    match user {
        // If the user is found, return the user data as JSON
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        // If no user is found with the given ID, return a 404 response
        Ok(None) => HttpResponse::NotFound().finish(),
        // If there's an error during the query, return an internal server error response
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Configures the routes for user-related endpoints.
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/create", web::post().to(create_user))  // Route to create a new user
            .route("/{id}", web::get().to(get_user)),      // Route to fetch a user by ID
    );
}

