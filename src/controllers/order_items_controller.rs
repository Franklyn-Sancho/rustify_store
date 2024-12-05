use std::sync::Arc;

use actix_web::{web, HttpResponse, Error};
use rust_decimal::Decimal;
use tokio_postgres::Client;
use uuid::Uuid;
use serde::Deserialize;

use crate::models::{order_items_model::OrderItem};

/// Represents the request body to create an order item, including the product ID, quantity, and price.
#[derive(Deserialize)]
pub struct CreateOrderItemRequest {
    pub product_id: Uuid, // Product ID
    pub quantity: i32,    // Product quantity
}


/// Handler to create a new order item.
pub async fn create_order_item(
    client: web::Data<Arc<Client>>,
    order_id: web::Path<Uuid>,
    body: web::Json<CreateOrderItemRequest>,
) -> Result<HttpResponse, Error> {
    let order_item = OrderItem::create_order_item(
        &client,
        *order_id,
        body.product_id,
        body.quantity,
    )
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(order_item))
}



/// Handler to retrieve all items for a specific order.
pub async fn get_order_items(
    client: web::Data<Arc<Client>>,  // Database client
    order_id: web::Path<Uuid>,      // Order ID to fetch the items for
) -> Result<HttpResponse, Error> {
    // Fetch order items using the order ID
    let items = OrderItem::get_order_items(&client, *order_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Return the list of items for the order as a JSON response
    Ok(HttpResponse::Ok().json(items))
}

/// Handler to delete an order item.
pub async fn delete_order_item(
    client: web::Data<Arc<Client>>, // Database client
    item_id: web::Path<Uuid>,      // Order item ID to delete
) -> Result<HttpResponse, Error> {
    // Attempt to delete the order item
    let success = OrderItem::delete_order_item(&client, *item_id)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    // Return appropriate response based on deletion success
    if success {
        Ok(HttpResponse::NoContent().finish()) // No content if deletion is successful
    } else {
        Ok(HttpResponse::NotFound().finish()) // Not found if the item does not exist
    }
}

