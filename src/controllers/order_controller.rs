use std::sync::Arc;

use tokio_postgres::{Client, Error};
use uuid::Uuid;
use actix_web::{web, HttpResponse};

use crate::models::order_models::Order;

#[derive(serde::Deserialize)]
pub struct CreateOrderRequest {
    pub status: String,
}

/// Handler para criar um pedido
pub async fn create_order(
    client: web::Data<Arc<Client>>, // Cliente do banco de dados
    user_id: web::Path<Uuid>, // ID do usuário como parâmetro
    body: web::Json<CreateOrderRequest>, // Dados do pedido recebidos no corpo da requisição
) -> HttpResponse {
    let order = Order::create_order(
        &client,
        *user_id,
        body.status.clone(), // Aqui você pode extrair o status do corpo da requisição
    )
    .await;

    match order {
        // If order creation is successful, return the order data as JSON in the response
        Ok(order) => HttpResponse::Ok().json(order),
        // If an error occurs during order creation, log the error and return a 500 internal server error
        Err(e) => {
            eprintln!("Error creating order: {:?}", e); // Log the error details
            HttpResponse::InternalServerError().finish() // Send a generic internal server error response
        }
    }
}

/// Handler para obter um pedido
pub async fn get_order(
    client: web::Data<Arc<Client>>, // Cliente do banco de dados
    order_id: web::Path<Uuid>, // ID do pedido como parâmetro
) -> HttpResponse {
    let order = Order::get_order(&client, *order_id).await;

    match order {
        // If the order is found, return the order data as JSON
        Ok(Some(order)) => HttpResponse::Ok().json(order),
        // If no order is found with the given ID, return a 404 response
        Ok(None) => HttpResponse::NotFound().finish(),
        // If there's an error during the query, return an internal server error response
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Handler para deletar um pedido
pub async fn delete_order(
    client: web::Data<Arc<Client>>, // Cliente do banco de dados
    order_id: web::Path<Uuid>, // ID do pedido como parâmetro
) -> HttpResponse {
    // Call the `delete_order` method from the order model to delete the order
    match Order::delete_order(&client, order_id.into_inner()).await {
        // If the deletion is successful, return a 204 No Content response
        Ok(_) => HttpResponse::NoContent().finish(),
        // If an error occurs during deletion, return an internal server error
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


