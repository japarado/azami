use crate::database::StatePool;
use crate::models::user::User;
use crate::schema::tags;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};

#[derive(Serialize, Deserialize, Identifiable, Queryable, AsChangeset, Associations, PartialEq, Debug)]
#[belongs_to(User)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset, PartialEq, Debug)]
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

    pub fn show(pool: StatePool, pk: &i32) -> Result<Self, Error> {
        use crate::schema::tags::dsl::*;
        let conn = &pool.get().unwrap();
        tags.find(pk).first::<Self>(conn)
    }

    pub fn update(pool: StatePool, new_tag: NewTag, pk: &i32) -> Result<Self, Error> {
        use crate::schema::tags::dsl::*;
        let conn = &pool.get().unwrap();
        let target = tags.find(pk);
        diesel::update(target).set(new_tag).get_result::<Tag>(conn)
    }

    pub fn destroy(pool: StatePool, pk: &i32, user_fk: &i32) -> Result<Self, Error> {
        use crate::schema::tags::dsl::*;
        let conn = &pool.get().unwrap();
        let target = tags.find(pk).filter(user_id.eq(user_fk));
        diesel::delete(target).get_result(conn)
    }

    pub fn my_tags(pool: StatePool, user_fk: &i32) -> Result<Vec<Self>, Error> {
        use crate::schema::tags::dsl::*;
        let conn = &pool.get().unwrap();
        let tag_owner = User::show(pool, user_fk)?;
        Tag::belonging_to(&tag_owner).get_results(conn)
    }
}

// impl Responder for Tag {
//     type Error = actix_web::Error;
//     type Future = Ready<Result<HttpResponse, actix_web::Error>>;

//     fn respond_to(self, _req: &HttpRequest) -> Self::Future {
//         let body = serde_json::to_string(&self).unwrap();

//         // Create response and set content type
//         ready(Ok(HttpResponse::Ok()
//             .content_type("application/json")
//             .body(body)))
//     }
// }
