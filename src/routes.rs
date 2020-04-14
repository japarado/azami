use crate::controllers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/posts")
            .service(controllers::post_controller::all)
            .service(controllers::post_controller::index)
            .service(controllers::post_controller::store)
            .service(controllers::post_controller::destroy)
            .service(controllers::post_controller::show)
            // .service(controllers::post_controller::demo_store)
            // .service(controllers::post_controller::demo_destroy),
    );

    cfg.service(
        web::scope("/users").service(controllers::user_controller::index), // .service(controllers::user_controller::delete_all),
    );

    cfg.service(
        web::scope("/auth")
            .service(controllers::auth_controller::register)
            .service(controllers::auth_controller::login)
            .service(controllers::auth_controller::me)
            .service(controllers::auth_controller::logout),
    );

    cfg.service(
        web::scope("/tags")
            .service(controllers::tag_controller::index)
            .service(controllers::tag_controller::show)
            .service(controllers::tag_controller::store)
            .service(controllers::tag_controller::update)
            .service(controllers::tag_controller::destroy),
    );

    cfg.service(web::scope("/post_tags").service(controllers::post_tag_controller::index));
}
