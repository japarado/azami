use crate::database::Pool;
use crate::models::user;
use crate::models::user::{AuthData, SlimUser, User};
use actix_identity::Identity;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[post("/login")]
pub async fn login(
    auth_data: web::Form<AuthData>,
    id: Identity,
    pool: web::Data<Pool>,
) -> impl Responder {
    let verification_res = web::block(move || {
        user::auth::verify_user(AuthData::new(&auth_data.email, &auth_data.password), pool)
    })
    .await;

    println!("{:?}", verification_res);

    match verification_res {
        Ok(slim_user) => {
            let user_string = serde_json::to_string(&slim_user).unwrap();
            id.remember(user_string.clone());
            HttpResponse::Ok().json(responders::UserLoggedIn {
                user: slim_user,
                token: user_string,
            })
        }
        Err(_err) => HttpResponse::Ok().json("Invalid credentials"),
    }
}

#[post("/register")]
pub async fn register(pool: web::Data<Pool>, auth_data: web::Form<AuthData>) -> impl Responder {
    web::block(move || -> Result<User, diesel::result::Error> { Ok(User::store(pool, auth_data)?) })
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::Ok().json("User already exists"))
}

#[get("/me")]
pub async fn me(logged_user: SlimUser, id: Identity) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}

#[delete("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok().json("Logged out")
}

mod responders {
    use crate::models::user::SlimUser;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct UserLoggedIn {
        pub user: SlimUser,
        pub token: String,
    }
}
