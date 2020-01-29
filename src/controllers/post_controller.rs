use crate::controllers::IdPath;
use crate::database::StatePool;
use crate::models::post::{NewPost, Post};
use crate::models::user::AuthUser;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[get("/all")]
pub async fn all(pool: StatePool) -> impl Responder {
    web::block(move || -> Result<Vec<Post>, Error> { Ok(Post::index(pool)?) })
        .await
        .map(|posts| HttpResponse::Ok().json(Multiple { posts }))
        .map_err(|_| HttpResponse::InternalServerError())
}

#[get("")]
pub async fn index(pool: StatePool, auth_user: AuthUser) -> impl Responder {
    web::block(move || -> Result<Vec<Post>, Error> { Ok(Post::my_posts(pool, &auth_user.id)?) })
        .await
        .map(|posts| HttpResponse::Ok().json(Multiple { posts }))
        .map_err(|_| HttpResponse::InternalServerError())
}

#[post("")]
pub async fn store(
    pool: StatePool,
    form: web::Form<RequestPost>,
    auth_user: AuthUser,
) -> impl Responder {
    web::block(move || -> Result<Post, Error> {
        let new_post = NewPost {
            title: form.title.to_string(),
            body: form.body.to_string(),
            user_id: auth_user.id.to_owned(),
        };
        Ok(Post::store(pool, new_post)?)
    })
    .await
    .map(|post| HttpResponse::Created().json(post))
    .map_err(|_| HttpResponse::InternalServerError())
}

#[delete("/{id}")]
pub async fn destroy(
    pool: StatePool,
    path: web::Path<IdPath>,
    auth_user: AuthUser,
) -> impl Responder {
    web::block(move || -> Result<Post, Error> { Ok(Post::destroy(pool, &path.id, &auth_user.id)?) })
        .await
        .map(|post| HttpResponse::Ok().json(Single { post }))
        .map_err(|_| {
            HttpResponse::BadRequest().json("Post not found or not existing to current user");
        })
}

#[patch("/{id}")]
pub async fn update(
    pool: StatePool,
    path: web::Path<IdPath>,
    form: web::Form<NewPost>,
    auth_user: AuthUser,
) -> impl Responder {
    web::block(move || -> Result<Post, Error> {
        let new_post = NewPost {
            title: form.title.to_owned(),
            body: form.body.to_owned(),
            user_id: path.id.to_owned(),
        };
        Ok(Post::update(pool, new_post, &path.id, &auth_user.id)?)
    })
    .await
    .map(|post| HttpResponse::Ok().json(Single { post }))
    .map_err(|_| HttpResponse::InternalServerError())
}

#[get("/{id}")]
pub async fn show(pool: StatePool, path: web::Path<IdPath>, auth_user: AuthUser) -> impl Responder {
    web::block(move || -> Result<Post, Error> { Ok(Post::show(pool, &path.id)?) })
        .await
        .map(|post| HttpResponse::Ok().json(Single { post }))
        .map_err(|_| HttpResponse::InternalServerError())
}

#[derive(Serialize, Deserialize)]
pub struct RequestPost {
    pub title: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct Single {
    pub post: Post,
}

#[derive(Serialize)]
pub struct Multiple {
    pub posts: Vec<Post>,
}
