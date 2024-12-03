use actix_web::web;

use crate::controllers::order_controller::{create_order, delete_order, get_order};

pub fn order_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/orders/{user_id}")
            .route(web::post().to(create_order))
            .route(web::get().to(get_order)),
    )
    .service(web::resource("/orders/{order_id}").route(web::delete().to(delete_order)));
}
