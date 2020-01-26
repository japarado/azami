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
        .map(|posts| HttpResponse::Ok().json(responders::Multiple { posts }))
        .map_err(|_| HttpResponse::InternalServerError())
}

#[get("")]
pub async fn index(pool: StatePool, auth_user: AuthUser) -> impl Responder {
    web::block(move || -> Result<Vec<Post>, Error> { Ok(Post::my_posts(pool, &auth_user.id)?) })
        .await
        .map(|posts| HttpResponse::Ok().json(posts))
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

#[derive(Serialize, Deserialize)]
pub struct RequestPost {
    pub title: String,
    pub body: String,
}

mod responders {
    use crate::models::post::Post;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Single {
        pub post: Post,
    }

    #[derive(Serialize)]
    pub struct Multiple {
        pub posts: Vec<Post>,
    }
}
