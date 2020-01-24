use crate::schema::tags;
use actix_web::Responder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Identifiable, Queryable, PartialEq, Debug)]
pub struct Tag {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Insertable, PartialEq, Debug)]
#[table_name = "tags"]
pub struct NewTag {
    name: String,
}
