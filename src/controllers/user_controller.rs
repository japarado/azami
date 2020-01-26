use crate::database::StatePool;
use crate::models::post::Post;
use crate::models::user::{AuthUser, User};
use actix_web::{get, web, HttpResponse, Responder};
use diesel::result::Error;

#[get("")]
pub async fn index(pool: StatePool) -> impl Responder {
    web::block(move || -> Result<Vec<User>, Error> { Ok(User::index(pool)?) })
        .await
        .map(|posts| HttpResponse::Ok().json(posts))
        .map_err(|_| HttpResponse::InternalServerError())
}
