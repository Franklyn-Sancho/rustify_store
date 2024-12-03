use crate::models::payment_model::Payment;
use actix_web::{web, HttpResponse, Error};
use tokio_postgres::Client;
use uuid::Uuid;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePaymentRequest {
    pub payment_method: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct UpdatePaymentStatusRequest {
    pub status: String,
}

/// Handler to create a payment
pub async fn create_payment(
    client: web::Data<Client>,
    order_id: web::Path<Uuid>,
    body: web::Json<CreatePaymentRequest>,
) -> Result<HttpResponse, Error> {
    let payment = Payment::create_payment(
        &client,
        *order_id,
        body.payment_method.clone(),
        body.status.clone(),
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(payment))
}

/// Handler to retrieve a payment by order ID
pub async fn get_payment(
    client: web::Data<Client>,
    order_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let payment = Payment::get_payment(&client, *order_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(payment) = payment {
        Ok(HttpResponse::Ok().json(payment))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

/// Handler to update payment status
pub async fn update_payment_status(
    client: web::Data<Client>,
    payment_id: web::Path<Uuid>,
    body: web::Json<UpdatePaymentStatusRequest>,
) -> Result<HttpResponse, Error> {
    let success = Payment::update_payment_status(&client, *payment_id, body.status.clone())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if success {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
