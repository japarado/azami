use crate::database::Pool;
use crate::models::user::User;
use crate::schema::posts;
use actix_web::web;
use diesel::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub user_id: i32,
}
#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

impl Post {
    pub fn all(pool: web::Data<Pool>, owner_fk: i32) -> Result<Vec<Post>, diesel::result::Error> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        posts
            .filter(user_id.eq(owner_fk))
            .order(id.asc())
            .load::<Post>(conn)
    }
}

impl NewPost {
    pub fn new(title: &str, body: &str, user_id: &i32) -> Self {
        Self {
            title: String::from(title),
            body: String::from(body),
            user_id: user_id.to_owned(),
        }
    }
}
