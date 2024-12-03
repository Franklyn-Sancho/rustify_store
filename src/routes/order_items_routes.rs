use actix_web::web;

use crate::controllers::order_items_controller::{
    create_order_item, get_order_items, delete_order_item,
};

pub fn order_item_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/orders/{order_id}/items")
            .route(web::post().to(create_order_item))
            .route(web::get().to(get_order_items)),
    )
    .service(web::resource("/order_items/{item_id}").route(web::delete().to(delete_order_item)));
}
