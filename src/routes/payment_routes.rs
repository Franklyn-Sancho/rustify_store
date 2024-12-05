use actix_web::web;

use crate::controllers::payment_controller::{
    create_payment, get_payment, update_payment
};

pub fn payment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/payments/orders/{order_id}")
            .route(web::post().to(create_payment))
            .route(web::get().to(get_payment)),
    )
    .service(
        web::resource("/payments/{payment_id}")
            .route(web::patch().to(update_payment)),
    );
}
