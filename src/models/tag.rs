use crate::database::StatePool;
use crate::schema::tags;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, PartialEq, Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Insertable, PartialEq, Debug)]
#[table_name = "tags"]
pub struct NewTag {
    pub name: String,
    pub user_id: i32,
}

impl Tag {
    pub fn index(pool: StatePool) -> Result<Vec<Self>, Error> {
        use crate::schema::tags::dsl::*;
        let conn = &pool.get().unwrap();
        tags.order(id.asc()).load(conn)
    }

    pub fn store(pool: StatePool, new_tag: NewTag) -> Result<Self, Error> {
        use crate::schema::tags::dsl::*;
        let conn = &pool.get().unwrap();
        diesel::insert_into(tags)
            .values(new_tag)
            .get_result::<Self>(conn)
    }

    // pub fn my_tags(pool: StatePool) -> Result<Vec<Self>, Error> {
    //     use crate::schema::posts::tags::dsl::*;
    // }
}