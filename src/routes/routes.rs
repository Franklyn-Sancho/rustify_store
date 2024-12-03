use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use super::{order_items_routes, order_routes::{self, order_routes}, product_routes, user_routes};

// Health check endpoint to verify if the server is running
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is running!")
}


// Function to configure the routes for the application
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Configures the health check route and user routes
    cfg.route("/", web::get().to(health_check))
    .configure(user_routes::user_router) // Configures the user-related routes
    .configure(product_routes::product_router) // Configures the user-related routes
    .configure(order_routes::order_routes)
    .configure(order_items_routes::order_item_routes);
}