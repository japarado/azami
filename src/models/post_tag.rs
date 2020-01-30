use crate::database::StatePool;
use crate::models::{post::Post, tag::Tag};
use crate::schema::post_tags;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Insertable, Queryable, Identifiable, Associations, PartialEq, Debug,
)]
#[primary_key(tag_id, post_id)]
#[table_name = "post_tags"]
#[belongs_to(Tag)]
#[belongs_to(Post)]
pub struct PostTag {
    post_id: i32,
    tag_id: i32,
}

impl PostTag {
    pub fn index(pool: StatePool) -> Result<Vec<Self>, Error> {
        use crate::schema::post_tags::dsl::*;
        let conn = &pool.get().unwrap();
        post_tags.load(conn)
    }

    pub fn store(pool: StatePool, post_tag: Self) -> Result<Self, Error> {
        use crate::schema::post_tags::dsl::*;
        let conn = &pool.get().unwrap();
        diesel::insert_into(post_tags)
            .values(post_tag)
            .get_result(conn)
    }

    pub fn destroy(pool: StatePool, post_id: &i32, tag_id: &i32) -> Result<Self, Error> {
        use crate::schema::post_tags::dsl::*;
        let target = post_tags.find((tag_id, post_id));
        let conn = &pool.get().unwrap();
        diesel::delete(target).get_result(conn)
    }

    pub fn show(pool: StatePool, post_id: &i32, tag_id: &i32) -> Result<Self, Error> {
        use crate::schema::post_tags::dsl::*;
        let conn = &pool.get().unwrap();
        post_tags.find((tag_id, post_id)).get_result(conn)
    }
}
