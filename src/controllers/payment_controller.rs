use std::sync::Arc;

use crate::models::payment_model::Payment;
use actix_web::{web, HttpResponse, Error};
use log::{error, info};
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
    pub payment_method: String,  // O método de pagamento que o usuário deseja usar
}

/// Handler to create a payment for an order.
pub async fn create_payment(
    client: web::Data<Arc<Client>>,           // Database client
    order_id: web::Path<Uuid>,               // Associated order ID
    body: web::Json<CreatePaymentRequest>,   // Request body with payment method
) -> Result<HttpResponse, Error> {
    // Create the payment for the order with the provided method
    let payment = Payment::create_payment(
        &client,
        *order_id,
        &body.payment_method,
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // Return the created payment in the response
    Ok(HttpResponse::Created().json(payment))
}

/// Handler to retrieve the payment for a specific order.
pub async fn get_payment(
    client: web::Data<Arc<Client>>,  // Database client
    order_id: web::Path<Uuid>,      // Order ID to fetch the payment for
) -> Result<HttpResponse, Error> {
    // Fetch the payment associated with the given order ID
    let payment = Payment::get_payment(&client, *order_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // If payment exists, return it as a JSON response, otherwise return 404
    if let Some(payment) = payment {
        Ok(HttpResponse::Ok().json(payment))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

/// Handler to update the payment status.
pub async fn update_payment(
    client: web::Data<Arc<Client>>,
    payment_id: web::Path<Uuid>,
    body: web::Json<UpdatePaymentRequest>,
) -> Result<HttpResponse, Error> {
    // Update the payment method
    let method_updated = Payment::update_payment_method(&client, *payment_id, &body.payment_method).await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if method_updated {
        // After updating the payment method, update the status to "paid"
        Payment::update_payment_status(&client, *payment_id)
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::NoContent().finish())  // Returns 204 No Content if the operation is successful
    } else {
        Ok(HttpResponse::NotFound().finish())  // Returns 404 if the payment is not found
    }
}







