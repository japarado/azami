use crate::database::Pool;
use crate::models::user::User;
use crate::schema::posts;
use actix_web::web;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestPost {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub user_id: i32,
}
#[derive(Serialize, Deserialize, Queryable, Identifiable, Associations, PartialEq, Debug)]
#[belongs_to(User)]
#[table_name = "posts"]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

impl Post {
    pub fn all(pool: web::Data<Pool>) -> Result<Vec<Post>, diesel::result::Error> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        posts.order(id.asc()).load::<Post>(conn)
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
    pub fn new(title: &str, body: &str, user_id: &i32) -> Self {
        Self {
            title: title.to_string(),
            body: body.to_string(),
            user_id: user_id.to_owned(),
        }
    }
}
