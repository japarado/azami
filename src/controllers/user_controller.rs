use crate::database::Pool;
use crate::models::user::{responders, SlimUser, User};
use actix_identity::Identity;
use actix_web::{delete, get, web, HttpResponse, Responder};
use diesel::result::Error;

#[get("")]
pub async fn index(
    pool: web::Data<Pool>,
    identity: Identity,
    logged_user: SlimUser,
) -> impl Responder {
    web::block(move || -> Result<Vec<User>, Error> { Ok(User::all(pool)?) })
        .await
        .map(|users| HttpResponse::Ok().json(responders::Multiple { users }))
        .map_err(|_| HttpResponse::InternalServerError().json("Error retrieving all users"))
}
