use crate::controllers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(controllers::auth_controller::login)
            .service(controllers::auth_controller::me)
            .service(controllers::auth_controller::register),
    );

    cfg.service(
        web::scope("/users")
            .service(controllers::user_controller::index)
    );
}
