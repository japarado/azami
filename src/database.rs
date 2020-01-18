use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn create_pool() -> Pool {
    let connspec = env::var("DATABASE_URL").expect("DATABASE_URL expected in .env");
    let manager = ConnectionManager::<PgConnection>::new(connspec);

    r2d2::Pool::builder()
        .build(manager)
        .expect("Fatal Error: Failed to create pool")
}

// use actix::prelude::*;
// use diesel::result::Error;
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize)]
// pub struct User {
//     id: i32,
//     name: String,
//     hash: String,
// }

// struct DbExecutor(PgConnection);

// impl Actor for DbExecutor {
//     type Context = SyncContext<Self>;
// }
// #[derive(Serialize, Deserialize)]
// struct CreateUser {
//     email: String,
//     hash: String,
// }

// impl Message for CreateUser {
//     type Result = Result<User, Error>;
// }

// impl Handler<CreateUser> for DbExecutor {
//     type Result = Result<User, Error>;

//     fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
//         use crate::schema::users::dsl::*;
//     }
// }
