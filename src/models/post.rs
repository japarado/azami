use crate::database::StatePool;
use crate::models::user::User;
use crate::schema::posts;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable, PartialEq, Debug)]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Associations, Queryable, Identifiable, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub user_id: i32,
}

impl Post {
    // Returns all posts owned by the current auth user
    pub fn index(pool: StatePool) -> Result<Vec<Self>, Error> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        posts.order(id.asc()).load::<Self>(conn)
    }

    pub fn store(pool: StatePool, new_post: NewPost) -> Result<Self, Error> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        diesel::insert_into(posts).values(new_post).get_result(conn)
    }

    // Returns all posts owned by the current auth user
    pub fn my_posts(pool: StatePool, user_fk: &i32) -> Result<Vec<Self>, Error> {
        let conn = &pool.get().unwrap();
        let target_user = User::show(pool, user_fk)?;
        Post::belonging_to(&target_user).load::<Self>(conn)
    }
}

impl NewPost {
    pub fn new() -> Self {
        Self {
            title: String::from(""),
            body: String::from(""),
            user_id: -1,
        }
    }
}
