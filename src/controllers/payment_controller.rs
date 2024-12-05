use std::sync::Arc;

use crate::{auth::AuthenticatedUser, models::{order_model::Order, payment_model::Payment}};
use actix_web::{web, HttpResponse, Error};
use tokio_postgres::Client;
use uuid::Uuid;
use serde::Deserialize;

// Represents the request body to create a payment, including the payment method.
#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    pub payment_method: String, // "credit_card", "paypal"
}

/// Represents the request body to update the payment status.
#[derive(Deserialize)]
pub struct UpdatePaymentRequest {
    pub payment_method: String, 
}

pub async fn create_payment(
    client: web::Data<Arc<Client>>,           // Database client
    auth_user: AuthenticatedUser,            // Authenticated user
    order_id: web::Path<Uuid>,               // Associated order ID
    body: web::Json<CreatePaymentRequest>,   // Request body with payment method
) -> Result<HttpResponse, Error> {
    // Validate if the order belongs to the authenticated user
    let user_id = auth_user.0.sub;
    let is_owner = Order::verify_order_owner(&client, *order_id, user_id).await;

    if let Err(_) | Ok(false) = is_owner {
        return Ok(HttpResponse::Forbidden().body("You do not have permission to access this order."));
    }

    // Create the payment
    let payment = Payment::create_payment(
        &client,
        *order_id,
        &body.payment_method,
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(payment))
}



/// Handler to retrieve the payment for a specific order.
pub async fn get_payment(
    client: web::Data<Arc<Client>>,  // Database client
    auth_user: AuthenticatedUser,   // Authenticated user
    order_id: web::Path<Uuid>,      // Order ID to fetch the payment for
) -> Result<HttpResponse, Error> {
    // validate user
    let user_id = auth_user.0.sub;
    let is_owner = Order::verify_order_owner(&client, *order_id, user_id).await;

    if let Err(_) | Ok(false) = is_owner {
        return Ok(HttpResponse::Forbidden().body("You do not have permission to access this order."));
    }

    // get payment
    let payment = Payment::get_payment(&client, *order_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(payment) = payment {
        Ok(HttpResponse::Ok().json(payment))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}


/// Handler to update the payment status.
pub async fn update_payment(
    client: web::Data<Arc<Client>>,        // Database client
    auth_user: AuthenticatedUser,         // Authenticated user
    payment_id: web::Path<Uuid>,          // Payment ID
    body: web::Json<UpdatePaymentRequest>, // Request body
) -> Result<HttpResponse, Error> {
    // validate user
    let user_id = auth_user.0.sub;

    let is_owner = Payment::verify_payment_owner(&client, &payment_id, user_id).await;

    if let Err(_) | Ok(false) = is_owner {
        return Ok(HttpResponse::Forbidden().body("You do not have permission to access this payment."));
    }

    // update payment method
    let method_updated = Payment::update_payment_method(&client, *payment_id, &body.payment_method)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if method_updated {
        Payment::update_payment_status(&client, *payment_id)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}








