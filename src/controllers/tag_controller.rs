use crate::database::StatePool;
use crate::models::tag::{NewTag, Tag};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::result::Error;
use serde::{ Serialize, Deserialize };

#[get("")]
pub async fn index(pool: StatePool) -> impl Responder {
    web::block(move || -> Result<Vec<Tag>, Error> {
        use crate::schema::tags::dsl::*;
        let conn = &pool.get().unwrap();
        Ok(tags.order(id.asc()).load::<Tag>(conn)?)
    })
    .await
    .map(|tags| HttpResponse::Ok().json(Multiple { tags }))
    .map_err(|_| HttpResponse::InternalServerError())
}


#[derive(Serialize, Deserialize)]
pub struct Single {
    pub tag: Tag
}

#[derive(Serialize, Deserialize)]
pub struct Multiple {
    pub tags: Vec<Tag>
}
