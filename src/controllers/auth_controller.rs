use crate::database::StatePool;
use crate::models::user::{AuthUser, NewUser, User};
use actix_identity::Identity;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use serde_json;

#[post("/register")]
pub async fn register(pool: StatePool, form: web::Form<NewUser>) -> impl Responder {
    web::block(move || -> Result<User, Error> {
        Ok(User::store(
            pool,
            NewUser {
                email: form.email.to_owned(),
                password: form.password.to_owned(),
            },
        )?)
    })
    .await
    .map(|post| HttpResponse::Ok().json(post))
    .map_err(|_| HttpResponse::BadRequest().json("User already exists"))
}

#[post("/login")]
pub async fn login(pool: StatePool, form: web::Form<NewUser>, id: Identity) -> impl Responder {
    let verify_user_res = User::verify(pool, &form.email, &form.password).await;

    match verify_user_res {
        Ok(verified_user) => {
            let user_string = serde_json::to_string(&verified_user).unwrap();
            id.remember(user_string);
            HttpResponse::Ok().json(AuthUserResponse { user: verified_user })
        }
        Err(_err) => HttpResponse::BadRequest().json("Invalid Credentials"),
    }
}

#[get("/me")]
pub async fn me(auth_user: AuthUser) -> impl Responder {
    HttpResponse::Ok().json(AuthUserResponse { user: auth_user })
}

#[delete("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().json("Logged Out")
}

#[derive(Serialize, Deserialize)]
pub struct Single {
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct Multiple {
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthUserResponse {
    pub user: AuthUser,
}
