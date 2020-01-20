use crate::database::Pool;
use crate::models::post::{NewPost, Post};
use crate::models::user::SlimUser;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

#[get("")]
pub async fn index(pool: web::Data<Pool>, user: SlimUser) -> impl Responder {
    HttpResponse::Ok().json("terminator salvation")
}
