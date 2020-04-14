#[macro_use]
extern crate diesel;

extern crate argonautica;

use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::errhandlers::ErrorHandlers;
use actix_web::{get, http, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod controllers;
mod database;
mod error_handlers;
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
            .wrap(middleware::say_hi_middleware::SayHi)
            .wrap(Cors::new().supports_credentials().max_age(3600).finish())
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::UNAUTHORIZED, error_handlers::render_401),
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::get_secret_key().as_bytes())
                    .name("auth-cookie")
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

    let use_https: bool = false;
    if use_https {
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("key.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("cert.pem").unwrap();

        server.bind_openssl("localhost:8000", builder)?.run().await
    } else {
        server.run().await
    }

    // if env::var("HTTPS").unwrap_or_else(|_| -> String { String::from("true") }) == "true" {
    //     let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    //     builder
    //         .set_private_key_file("key.pem", SslFiletype::PEM)
    //         .unwrap();
    //     builder.set_certificate_chain_file("cert.pem").unwrap();

    //     server.bind_openssl("localhost:8000", builder)?.run().await
    // } else {
    //     server.run().await
    // }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("API Root")
}

#[get("/admin")]
async fn admin() -> impl Responder {
    HttpResponse::Ok().json("Admin Page")
}
