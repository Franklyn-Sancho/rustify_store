use std::sync::Arc;

use actix_web::{web, HttpResponse};
use tokio_postgres::Client;
use uuid::Uuid;
use serde::Deserialize;

use crate::models::user::User;

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}

pub async fn create_user(
    client: web::Data<Arc<Client>>,
    user_data: web::Json<CreateUserRequest>,
) -> HttpResponse {
    println!("Received request to create user: {:?}", user_data);

    let user = User::create_user(&client, &user_data.name, &user_data.email, &user_data.password).await;

    match user {
        Ok(user) => {
            HttpResponse::Ok().json(user)
        },
        Err(e) => {
            eprintln!("Error creating user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


pub async fn get_user(client: web::Data<Arc<Client>>, user_id: web::Path<Uuid>) -> HttpResponse {
    let user = User::get_user(&client, user_id.into_inner()).await;

    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/create", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user)),
    );
}
