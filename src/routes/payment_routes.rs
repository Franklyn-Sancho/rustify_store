use actix_web::web;

use crate::controllers::payment_controller::{
    create_payment, get_payment, update_payment_status,
};

pub fn payment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/payments/orders/{order_id}")
            .route(web::post().to(create_payment))
            .route(web::get().to(get_payment)),
    )
    .service(
        web::resource("/payments/{payment_id}")
            .route(web::put().to(update_payment_status)),
    );
}
