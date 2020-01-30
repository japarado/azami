use crate::database::StatePool;
use crate::models::post::Post;
use crate::models::user::{AuthUser, User};
use actix_web::{delete, get, web, HttpResponse, Responder};
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[get("")]
pub async fn index(pool: StatePool) -> impl Responder {
    web::block(move || -> Result<Vec<User>, Error> { Ok(User::index(pool)?) })
        .await
        .map(|posts| HttpResponse::Ok().json(posts))
        .map_err(|_| HttpResponse::InternalServerError())
}

// #[delete("/all")]
// pub async fn delete_all(pool: StatePool) -> impl Responder {
//     web::block(move || -> Result<Vec<User>, Error> {
//         use crate::schema::users::dsl::*;
//         let conn = &pool.get().unwrap();
//         Ok(diesel::delete(users).load(conn)?)
//     })
//     .await
//     .map(|users| HttpResponse::Ok().json(Multiple { users }))
//     .map_err(|_| HttpResponse::InternalServerError())
// }

#[derive(Serialize, Deserialize)]
pub struct Single {
    user: User,
}

#[derive(Serialize, Deserialize)]
pub struct Multiple {
    users: Vec<User>,
}
