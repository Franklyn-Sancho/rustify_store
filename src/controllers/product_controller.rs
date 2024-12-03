use std::sync::Arc;

use actix_web::{web, HttpResponse};
use rust_decimal::Decimal;
use serde::Deserialize;
use tokio_postgres::Client;
use uuid::Uuid;

use crate::models::product_model::Product;

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
}

pub async fn create_product(
    client: web::Data<Arc<Client>>, // Database client for interacting with the database
    product_data: web::Json<CreateProductRequest>, // The data submitted in the product creation request
) -> HttpResponse {

    // Attempt to create the product in the database
    let product = Product::create_product(
        &client,
        &product_data.name,
        product_data.description.as_deref(), // Converte Option<String> para Option<&str>
        product_data.price,
        product_data.stock,
    )
    .await;

    match product {
        // If product creation is successful, return the product data as JSON in the response
        Ok(product) => HttpResponse::Ok().json(product),
        // If an error occurs during product creation, log the error and return a 500 internal server error
        Err(e) => {
            eprintln!("Error creating product: {:?}", e); // Log the error details
            HttpResponse::InternalServerError().finish() // Send a generic internal server error response
        }
    }
}

pub async fn get_product(client: web::Data<Arc<Client>>, product_id: web::Path<Uuid>) -> HttpResponse {
    // Call the `get_product` method from the product model to fetch the product data
    let product = Product::get_product(&client, product_id.into_inner()).await;

    match product {
        // If the product is found, return the product data as JSON
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        // If no product is found with the given ID, return a 404 response
        Ok(None) => HttpResponse::NotFound().finish(),
        // If there's an error during the query, return an internal server error response
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Handler function to delete a product by their ID.
/// This function deletes the product record from the database based on the provided ID.
pub async fn delete_product(product_id: web::Path<Uuid>, client: web::Data<Arc<Client>>) -> HttpResponse {
    // Call the `delete_product` method from the product model to delete the product
    match Product::delete_product(&client, product_id.into_inner()).await {
        // If the deletion is successful, return a 204 No Content response
        Ok(_) => HttpResponse::NoContent().finish(),
        // If an error occurs during deletion, return an internal server error
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
