use crate::database::StatePool;
use crate::models::user::User;
use crate::schema::posts;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Insertable, AsChangeset, PartialEq, Debug)]
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
    pub fn my_posts(pool: StatePool, user_fk: &i32) -> Result<Vec<Self>, Error> {
        let conn = &pool.get().unwrap();
        let target_user = User::show(pool, user_fk)?;
        Post::belonging_to(&target_user).load::<Self>(conn)
    }
    // Returns all posts regardless of the owner
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

    pub fn destroy(pool: StatePool, pk: &i32, user_fk: &i32) -> Result<Self, Error> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        let target_user = User::show(pool, user_fk)?;
        let target = Post::belonging_to(&target_user).filter(id.eq(pk));
        diesel::delete(target).get_result(conn)
    }

    pub fn update(
        pool: StatePool,
        new_post: NewPost,
        pk: &i32,
        user_fk: &i32,
    ) -> Result<Self, Error> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        let target_user = User::show(pool, user_fk)?;
        let target_post = Post::belonging_to(&target_user).filter(id.eq(pk));
        diesel::update(target_post)
            .set(new_post)
            .get_result::<Self>(conn)
    }

    pub fn show(pool: StatePool, pk: &i32) -> Result<Self, Error> {
        use crate::schema::posts::dsl::*;
        let conn = &pool.get().unwrap();
        posts.find(pk).first(conn)
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
