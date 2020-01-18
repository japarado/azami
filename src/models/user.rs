use crate::database::Pool;
use crate::schema::users;
use actix_web::web;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlimUser {
    pub id: i32,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
        }
    }
}

impl NewUser {
    pub fn new(email: &str, hash: &str) -> Self {
        Self {
            email: email.to_string(),
            hash: hash.to_string(),
        }
    }
}

impl User {
    pub fn all(pool: web::Data<Pool>) -> Result<Vec<User>, Error> {
        use crate::schema::users::dsl::*;
        let conn = &pool.get().unwrap();
        users.order(id.asc()).load::<User>(conn)
    }

    pub fn store(pool: web::Data<Pool>, user: web::Form<AuthData>) -> Result<User, Error> {
        use crate::schema::users::dsl::*;
        let conn = &pool.get().unwrap();
        let hashed_password = crate::utils::hash_password(&user.password).unwrap();
        let new_user = NewUser::new(&user.email, &hashed_password);
        diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(conn)
    }
}

impl AuthData {
    pub fn new(email: &str, password: &str) -> Self {
        Self {
            email: email.to_string(),
            password: password.to_string(),
        }
    }
}

pub mod auth {
    use super::{AuthData, SlimUser, User};
    use crate::database::Pool;
    use crate::errors::AuthError;
    use crate::utils;
    use actix_web::web;
    use diesel::prelude::*;
    use diesel::result::Error;

    pub fn verify_user(auth_data: AuthData, pool: web::Data<Pool>) -> Result<SlimUser, AuthError> {
        let user = get_user_by_email(&auth_data.email, pool);

        match user {
            Ok(existing_user) => match utils::verify(&existing_user.hash, &auth_data.password) {
                Ok(_) => Ok(existing_user.into()),
                Err(_) => Err(AuthError::InvalidCredentials),
            },
            Err(_) => Err(AuthError::NotFound),
        }
    }

    fn get_user_by_email(email_query: &str, pool: web::Data<Pool>) -> Result<User, Error> {
        use crate::schema::users::dsl::*;
        let conn = &pool.get().unwrap();
        users.filter(email.eq(email_query)).first::<User>(conn)
    }
}

pub mod responders {
    use super::User;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Single {
        pub user: User,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Multiple {
        pub users: Vec<User>,
    }
}
