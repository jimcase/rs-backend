use actix_web::web;
use super::handlers::{create_user, get_user, update_user, delete_user, index};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("/{id}", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    )
    .route("/", web::get().to(index));
}
