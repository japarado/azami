use crate::controllers::IdPath;
use crate::database::Pool;
use crate::models::post::{NewPost, Post, RequestPost};
use crate::models::user::SlimUser;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

#[get("")]
pub async fn index(pool: web::Data<Pool>, auth_user: SlimUser) -> impl Responder {
    web::block(move || -> Result<Vec<Post>, diesel::result::Error> {
        Ok(Post::all_by_id(pool, &auth_user.id)?)
    })
    .await
    .map(|posts| HttpResponse::Ok().json(posts))
    .map_err(|_| HttpResponse::InternalServerError())
}

#[post("")]
pub async fn store(
    pool: web::Data<Pool>,
    form: web::Form<RequestPost>,
    auth_user: SlimUser,
) -> impl Responder {
    web::block(move || -> Result<Post, diesel::result::Error> {
        Ok(Post::store(
            pool,
            NewPost::new(&form.title, &form.body, &auth_user.id),
        )?)
    })
    .await
    .map(|post| HttpResponse::Ok().json(responders::Single { post }))
    .map_err(|_| HttpResponse::InternalServerError())
}

#[patch("/{id}")]
pub async fn update(
    pool: web::Data<Pool>,
    form: web::Form<RequestPost>,
    path: web::Form<IdPath>,
    auth_user: SlimUser,
) -> impl Responder {
    HttpResponse::Ok().json("Update Route")
}

#[delete("/{id}")]
pub async fn destroy(
    pool: web::Data<Pool>,
    path: web::Path<IdPath>,
    auth_user: SlimUser,
) -> impl Responder {
    web::block(move || -> Result<Post, diesel::result::Error> {
        Ok(Post::destroy(pool, &path.id, &auth_user.id)?)
    })
    .await
    .map(|post| HttpResponse::Ok().json(responders::Single { post }))
    .map_err(|_| HttpResponse::NotFound().json("Post not found/owned by requesting user"))
}

mod responders {
    use crate::models::post::Post;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Single {
        pub post: Post,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Multiple {
        pub posts: Vec<Post>,
    }
}
