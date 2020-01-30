use crate::controllers::MessageResponse;
use crate::database::StatePool;
use crate::models::post_tag::PostTag;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[get("/")]
pub async fn index(pool: StatePool) -> impl Responder {
    web::block(move || -> Result<Vec<PostTag>, Error> { Ok(PostTag::index(pool)?) })
        .await
        .map(|post_tags| HttpResponse::Ok().json(Multiple { post_tags }))
        .map_err(|err| {
            println!("{:?}", err);
            HttpResponse::InternalServerError().json(MessageResponse {
                message: format!("{:?}", err),
                success: false,
            })
        })
}

#[derive(Serialize, Deserialize)]
pub struct Multiple {
    post_tags: Vec<PostTag>,
}

#[derive(Serialize, Deserialize)]
pub struct Single {
    post_tag: PostTag,
}
