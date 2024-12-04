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
pub struct UpdatePaymentStatusRequest {
    pub status: String, // "completed", "failed"
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
pub async fn update_payment_status(
    client: web::Data<Arc<Client>>,           // Database client
    payment_id: web::Path<Uuid>,             // Payment ID to update
    body: web::Json<UpdatePaymentStatusRequest>, // Request body with new status
) -> Result<HttpResponse, Error> {
    info!("Updating payment status for payment_id: {}", payment_id);

    // Update the payment status
    let success = Payment::update_payment_status(&client, *payment_id, &body.status)
        .await
        .map_err(|e| {
            error!("Failed to update payment status: {:?}", e);
            actix_web::error::ErrorInternalServerError(e)
        })?;

    // Return appropriate response based on update success
    if success {
        info!("Payment status updated successfully for payment_id: {}", payment_id);
        Ok(HttpResponse::NoContent().finish()) // No content if update is successful
    } else {
        error!("Payment not found for payment_id: {}", payment_id);
        Ok(HttpResponse::NotFound().finish()) // Not found if the payment does not exist
    }
}


