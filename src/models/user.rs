use crate::database::StatePool;
use crate::errors;
use crate::schema::users;
use crate::utils;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

// Primary database model. User for queries
#[derive(Serialize, Deserialize, Identifiable, Queryable, PartialEq, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

// Stored in session for retrieving the authenticated user
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AuthUser {
    pub id: i32,
    pub email: String,
}

// Name might be misleading. NewUser is also used for logins
#[derive(Serialize, Deserialize, Insertable, PartialEq, Debug)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

impl User {
    pub fn show(pool: StatePool, pk: &i32) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;
        let conn = &pool.get().unwrap();
        users.find(pk).first::<Self>(conn)
    }

    pub fn store(pool: StatePool, new_user: NewUser) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;
        let conn = &pool.get().unwrap();

        let hashed_password = utils::hash_password(&new_user.password);
        diesel::insert_into(users)
            .values(NewUser {
                email: new_user.email.to_owned(),
                password: hashed_password.unwrap(),
            })
            .get_result::<Self>(conn)
    }

    pub async fn verify(
        pool: StatePool,
        email: &str,
        password: &str,
    ) -> Result<AuthUser, errors::AuthError> {
        let found_user = Self::get_user_by_email(pool, email).await;
        match found_user {
            Ok(existing_user) => {
                let is_verified = utils::verify(&existing_user.password, password)
                    .expect("Fatal error verifying password");
                if is_verified {
                    let auth_user: AuthUser = AuthUser {
                        id: existing_user.id,
                        email: existing_user.email,
                    };
                    Ok(auth_user)
                } else {
                    Err(errors::AuthError::InvalidCredentials)
                }
            }
            Err(_) => Err(errors::AuthError::NotFound),
        }
    }

    async fn get_user_by_email(pool: StatePool, email_query: &str) -> Result<Self, Error> {
        use crate::schema::users::dsl::*;
        let conn = &pool.get().unwrap();
        users.filter(email.eq(email_query)).first::<Self>(conn)
    }
}
