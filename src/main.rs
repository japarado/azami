#[macro_use]
extern crate diesel;

extern crate argonautica;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;

mod controllers;
mod database;
mod errors;
mod middleware;
mod models;
mod routes;
mod schema;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = database::create_pool();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Cors::new().finish())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::get_secret_key().as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(utils::get_domain().as_str())
                    .max_age(86400)
                    .secure(false),
            ))
            .data(pool.clone())
            .configure(routes::config)
            .service(index)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8000")?
    };

    server.run().await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("API Root")
}
