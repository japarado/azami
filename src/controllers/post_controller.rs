use crate::database::Pool;
use crate::models::post::{NewPost, Post, RequestPost};
use crate::models::user::SlimUser;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

// #[get("")]
// pub async fn index(pool: web::Data<Pool>, user: SlimUser) -> impl Responder {
//     web::block(move || {
//         use crate::models::user::User;
//         use crate::schema::users::dsl::{ users, posts };
//         use diesel::prelude::*;
//         let conn = &pool.get().unwrap();
//         let target_user = users.find(user.id).first::<User>(conn)?;
//         let user_posts = Post::belonging_to(target_user).load::<Post>(conn)?;
//         Ok(user_posts)
//     })
//     .await
//     .map(|posts| HttpResponse::Ok().json(posts))
//     .map_err(|_| HttpResponse::InternalServerError())
// }

#[get("")]
pub async fn index(pool: web::Data<Pool>) -> impl Responder {
    web::block(move || -> Result<Vec<Post>, diesel::result::Error> { Ok(Post::all(pool)?) })
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
    .map(|post| HttpResponse::Ok().json(post))
    .map_err(|_| HttpResponse::InternalServerError())
}
