use actix_web::web;

use crate::controllers::user_controller::{create_user, delete_user, get_user, login_user};

pub fn user_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/create", web::post().to(create_user))
            .route("/login", web::post().to(login_user))
            /* .route("/users/{user_id}", web::put().to(update_user)) */ 
            .route("/users/{user_id}", web::delete().to(delete_user)) // Excluir usuário
            .route("/{user_id}", web::get().to(get_user)),
    );
}
