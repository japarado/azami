#[macro_use]
extern crate diesel;

extern crate argonautica;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{get, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

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

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .max_age(3600)
                    .supports_credentials()
                    .disable_vary_header()
                    .finish(),
            )
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
            .service(admin)
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8000")?
    };


    server
        .bind_openssl("localhost:8000", builder)?
        .run().await
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("API Root")
}

#[get("/admin")]
async fn admin() -> impl Responder {
    HttpResponse::Ok().json("Admin Page")
}
