use crate::controllers::{IdPath, MessageResponse};
use crate::database::StatePool;
use crate::models::tag::{NewTag, Tag};
use crate::models::user::AuthUser;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[get("")]
pub async fn all(pool: StatePool) -> impl Responder {
    web::block(move || -> Result<Vec<Tag>, Error> { Ok(Tag::index(pool)?) })
        .await
        .map(|tags| HttpResponse::Ok().json(responders::Multiple { tags }))
}

#[post("")]
pub async fn store(
    pool: StatePool,
    form: web::Form<RequestTag>,
    auth_user: AuthUser,
) -> impl Responder {
    web::block(move || -> Result<Tag, Error> {
        let new_tag = NewTag {
            name: form.name.to_owned(),
            user_id: auth_user.id,
        };
        Ok(Tag::store(pool, new_tag)?)
    })
    .await
    .map(|tag| HttpResponse::Created().json(responders::Single { tag }))
    .map_err(|err| HttpResponse::InternalServerError().json(format!("{:?}", err)))
}

#[patch("/{id}")]
pub async fn update(
    pool: StatePool,
    form: web::Form<RequestTag>,
    path: web::Path<IdPath>,
    auth_user: AuthUser,
) -> impl Responder {
    web::block(move || -> Result<Tag, Error> {
        let new_tag = NewTag {
            name: form.name.to_owned(),
            user_id: auth_user.id.to_owned(),
        };
        Ok(Tag::update(pool, new_tag, &path.id)?)
    })
    .await
    .map(|tag| HttpResponse::Ok().json(responders::Single { tag }))
    .map_err(|err| HttpResponse::InternalServerError().json(format!("{:?}", err)))
}

#[delete("/")]
pub async fn destroy(
    pool: StatePool,
    path: web::Path<IdPath>,
    auth_user: AuthUser,
) -> impl Responder {
    web::block(move || -> Result<Tag, Error> { Ok(Tag::destroy(pool, &path.id)?) })
        .await
        .map(|tag| HttpResponse::Ok().json(responders::Single { tag }))
    .map_err(|err| HttpResponse::InternalServerError().json(format!("{:?}", err)))
}

#[derive(Serialize, Deserialize)]
pub struct RequestTag {
    name: String,
}

mod responders {
    use crate::models::tag::Tag;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Single {
        pub tag: Tag,
    }

    #[derive(Serialize)]
    pub struct Multiple {
        pub tags: Vec<Tag>,
    }
}
