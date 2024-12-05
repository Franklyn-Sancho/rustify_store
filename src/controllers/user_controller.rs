use std::sync::Arc;

use crate::{jwt::create_jwt, models::user_model::User};
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use tokio_postgres::Client;
use uuid::Uuid;

/// Struct representing the request data for creating a user.
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,    
    pub email: String,    
    pub password: String, 
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,    
    pub password: String, 
}

/// Handler function to create a user.
/// This function processes the incoming HTTP request, hashes the password, and
/// stores the user in the database.
pub async fn create_user(
    client: web::Data<Arc<Client>>,              // Database client for interacting with the database
    user_data: web::Json<CreateUserRequest>,     // The data submitted in the user creation request
) -> HttpResponse {
    // Check if the email is already in use by querying the database
    if User::email_exists(&client, &user_data.email).await.unwrap_or(false) {
        return HttpResponse::BadRequest().body("Email already in use");  // Return an error response if email is in use
    }

    // Attempt to create the user in the database
    let user = User::create_user(
        &client,
        &user_data.name,
        &user_data.email,
        &user_data.password, // The password will be hashed within the `create_user` function
    )
    .await;

    match user {
        // If user creation is successful, return the user data as JSON in the response
        Ok(user) => HttpResponse::Ok().json(user),
        // If an error occurs during user creation, log the error and return a 500 internal server error
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);  // Log the error details
            HttpResponse::InternalServerError().finish()  // Send a generic internal server error response
        }
    }
}

/// Handler function to authenticate a user by login credentials.
/// This function checks the provided credentials and generates a JWT token for the user if successful.
pub async fn login_user(
    client: web::Data<Arc<Client>>,              // Database client for interacting with the database
    login_data: web::Json<LoginRequest>,         // The data submitted in the login request
) -> HttpResponse {
    match User::authenticate_user(&client, &login_data.email, &login_data.password).await {
        // If authentication is successful, generate a JWT token for the authenticated user
        Ok(Some(user)) => {
            match create_jwt(user.id) {
                Ok(token) => HttpResponse::Ok().json(token),  // Return the JWT token as JSON
                Err(_) => HttpResponse::InternalServerError().body("Failed to generate token"),  // If token creation fails, return an error
            }
        }
        // If no user is found with the provided credentials, return an Unauthorized response
        Ok(None) => HttpResponse::Unauthorized().body("Invalid credentials"),
        // If an error occurs during authentication, return an internal server error
        Err(_) => HttpResponse::InternalServerError().body("An error occurred during login"),
    }
}

/// Handler function to fetch a user by their ID.
/// This function queries the database to retrieve the user based on the given ID.
pub async fn get_user(client: web::Data<Arc<Client>>, user_id: web::Path<Uuid>) -> HttpResponse {
    // Call the `get_user` method from the User model to fetch the user data
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

/// Handler function to delete a user by their ID.
/// This function deletes the user record from the database based on the provided ID.
pub async fn delete_user(user_id: web::Path<Uuid>, client: web::Data<Arc<Client>>) -> HttpResponse {
    // Call the `delete_user` method from the User model to delete the user
    match User::delete_user(&client, user_id.into_inner()).await {
        // If the deletion is successful, return a 204 No Content response
        Ok(_) => HttpResponse::NoContent().finish(),
        // If an error occurs during deletion, return an internal server error
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

