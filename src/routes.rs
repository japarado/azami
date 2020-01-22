use crate::controllers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(controllers::auth_controller::login)
            .service(controllers::auth_controller::me)
            .service(controllers::auth_controller::register)
            .service(controllers::auth_controller::logout)
            .service(controllers::auth_controller::alt_me),
    );

    cfg.service(
        web::scope("/posts")
            .service(controllers::post_controller::index)
            .service(controllers::post_controller::store),
    );

    cfg.service(
        web::scope("/users").service(controllers::user_controller::index), // .service(controllers::user_controller::delete_all_users),
    );
}
