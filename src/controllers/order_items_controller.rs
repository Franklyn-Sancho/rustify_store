use actix_web::{web, HttpResponse, Error};
use tokio_postgres::Client;
use uuid::Uuid;
use serde::Deserialize;

use crate::models::{order_items_models::OrderItem, order_models::Order};

#[derive(Deserialize)]
pub struct CreateOrderItemRequest {
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: f64,
}

/// Handler to create an order item
pub async fn create_order_item(
    client: web::Data<Client>,
    order_id: web::Path<Uuid>,
    body: web::Json<CreateOrderItemRequest>,
) -> Result<HttpResponse, Error> {
    let order_item = OrderItem::create_order_item(
        &client,
        *order_id,
        body.product_id,
        body.quantity,
        body.price,
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(order_item))
}

/// Handler to retrieve all items for a specific order
pub async fn get_order_items(
    client: web::Data<Client>,
    order_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let items = OrderItem::get_order_items(&client, *order_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(items))
}

/// Handler to delete an order item
pub async fn delete_order_item(
    client: web::Data<Client>,
    item_id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let success = OrderItem::delete_order_item(&client, *item_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if success {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}
