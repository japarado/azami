use crate::controllers::{IdPath, MessageResponse};
use crate::database::StatePool;
use crate::models::tag::{NewTag, Tag};
use crate::models::user::AuthUser;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[get("")]
pub async fn index(pool: StatePool) -> impl Responder {
    web::block(move || -> Result<Vec<Tag>, Error> { Ok(Tag::index(pool)?) })
        .await
        .map(|tags| HttpResponse::Ok().json(Multiple { tags }))
        .map_err(|err| {
            println!("{:?}", err);
            HttpResponse::InternalServerError().json(MessageResponse {
                message: format!("{:?}", err),
                success: false,
            })
        })
}

#[get("/{id}")]
pub async fn show(pool: StatePool, path: web::Path<IdPath>) -> impl Responder {
    web::block(move || -> Result<Tag, Error> { Ok(Tag::show(pool, &path.id)?) })
        .await
        .map(|tag| HttpResponse::Ok().json(Single { tag }))
        .map_err(|err| {
            println!("{:?}", err);
            HttpResponse::InternalServerError().json(MessageResponse {
                message: format!("{:?}", err),
                success: false,
            })
        })
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
            user_id: auth_user.id.to_owned(),
        };
        Ok(Tag::store(pool, new_tag)?)
    })
    .await
    .map(|tag| HttpResponse::Created().json(Single { tag }))
    .map_err(|err| {
        println!("{:?}", err);
        HttpResponse::InternalServerError().json(MessageResponse {
            message: format!("{:?}", err),
            success: false,
        })
    })
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
    .map(|tag| HttpResponse::Ok().json(Single { tag }))
    .map_err(|err| {
        println!("{:?}", err);
        HttpResponse::InternalServerError().json(MessageResponse {
            message: format!("{:?}", err),
            success: false,
        })
    })
}

#[derive(Deserialize, Serialize)]
pub struct RequestTag {
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Single {
    tag: Tag,
}

#[derive(Deserialize, Serialize)]
pub struct Multiple {
    tags: Vec<Tag>,
}
