use crate::database::Pool;
use actix_web::web;
use diesel::result::Error;

pub mod user;
pub mod post;

pub trait Model<T, U> {
    fn all(pool: web::Data<Pool>) -> Result<Vec<T>, Error>;
    fn show(pool: web::Data<Pool>, pk: i32) -> Result<T, Error>;
    fn store(pool: web::Data<Pool>, payload: web::Form<U>) -> Result<T, Error>;
    fn update(pool: web::Data<Pool>, pk: i32, payload: web::Form<U>) -> Result<T, Error>;
    fn delete(pool: web::Data<Pool>, pk: i32) -> Result<T, Error>;
}
