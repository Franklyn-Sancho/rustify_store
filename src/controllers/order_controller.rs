use std::sync::Arc;

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use tokio_postgres::Client;
use uuid::Uuid;

use crate::{auth::AuthenticatedUser, jwt::Claims, models::{order_items_model::OrderItem, order_model::Order, payment_model::Payment}};

/// Represents the request body to create a new order, including the items.
#[derive(Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub items: Vec<OrderItemRequest>, // List of items in the order
}

/// Represents an individual item in the order, including the product ID, quantity, and price.
#[derive(Serialize, Deserialize)]
pub struct OrderItemRequest {
    pub product_id: Uuid, // ID of the product being ordered
    pub quantity: i32,    // Quantity of the product
}

/// Handler to create an order along with its items and payment.
pub async fn create_order(
    client: web::Data<Arc<Client>>,      // Database client
    auth_user: AuthenticatedUser,           // User ID for the order
    body: web::Json<CreateOrderRequest>, // Request body containing order details
) -> HttpResponse {

    let user_id = auth_user.0.sub; 

    let order = match Order::create_order(&client, user_id).await {
        Ok(order) => order,
        Err(err) => {
            eprintln!("Error creating order: {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Processing the order items.
    for item in &body.items {
        let is_in_stock =
            match OrderItem::check_stock(&client, item.product_id, item.quantity).await {
                Ok(stock) => stock,
                Err(err) => {
                    eprintln!("Error checking stock: {:?}", err);
                    return HttpResponse::InternalServerError().finish();
                }
            };

        if !is_in_stock {
            return HttpResponse::BadRequest().body("Insufficient stock for one or more items.");
        }

        // Adding the item to the order.
        match OrderItem::create_order_item(&client, order.id, item.product_id, item.quantity).await
        {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error adding item to order: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    let payment_method = ""; // Empty payment method initially
    let _payment = match Payment::create_payment(&client, order.id, &payment_method).await {
        Ok(payment) => payment,
        Err(err) => {
            eprintln!("Error creating payment: {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    // Returning the order with the created payment (pending).
    HttpResponse::Created().json(order)
}


/// Handler to retrieve an order by its ID.
pub async fn get_order(
    client: web::Data<Arc<Client>>, // Database client
    order_id: web::Path<Uuid>,      // Order ID to fetch
) -> HttpResponse {
    let order = Order::get_order(&client, *order_id).await;

    match order {
        // If order is found, return as JSON
        Ok(Some(order)) => HttpResponse::Ok().json(order),
        // If no order is found, return a 404 Not Found response
        Ok(None) => HttpResponse::NotFound().finish(),
        // If an error occurs during the database query, return a 500 Internal Server Error
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Handler to delete an order by its ID.
pub async fn delete_order(
    client: web::Data<Arc<Client>>, // Database client
    order_id: web::Path<Uuid>,      // Order ID to delete
) -> HttpResponse {
    // Attempt to delete the order and handle any errors.
    match Order::delete_order(&client, order_id.into_inner()).await {
        // If the deletion is successful, return a 204 No Content response.
        Ok(_) => HttpResponse::NoContent().finish(),
        // If an error occurs, return a 500 Internal Server Error.
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
