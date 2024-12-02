use actix_web::web;

use crate::controllers::product_controller::{create_product, delete_product, get_product};



pub fn product_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("/create", web::post().to(create_product))
            /* .route("/users/{user_id}", web::put().to(update_user))  */
            .route("/users/{product_id}", web::delete().to(delete_product)) // Excluir usuário
            .route("/{product_id}", web::get().to(get_product)),
    );
}