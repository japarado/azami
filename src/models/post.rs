use crate::database::Pool;
use crate::schema::posts;
use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}
#[derive(Serialize, Deserialize, Queryable, Identifiable, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
}

impl Post {
    pub fn all(pool: web::Data<Pool>) -> Result<Vec<Post>, diesel::result::Error> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        posts.order(id.asc()).load::<Post>(conn)
    }

    pub fn show(pool: web::Data<Pool>, pk: i32) -> Result<Post, diesel::result::Error> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        posts.find(pk).first::<Post>(conn)
    }

    pub fn store(pool: web::Data<Pool>, new_post: NewPost) -> Result<Post, diesel::result::Error> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        diesel::insert_into(posts)
            .values(new_post)
            .get_result::<Post>(conn)
    }
}

impl NewPost {
    pub fn new(title: &str, body: &str) -> Self {
        Self {
            title: String::from(title),
            body: String::from(body),
        }
    }
}
