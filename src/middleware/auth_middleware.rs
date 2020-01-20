use crate::models::user::SlimUser;
use actix_identity::Identity;
use actix_web::{dev::Payload, error, FromRequest, HttpRequest};
use futures::future::Future;
use std::pin::Pin;

// pub type SlimUser = SlimUser;

impl FromRequest for SlimUser {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<SlimUser, actix_web::Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let fut = Identity::from_request(req, pl);

        Box::pin(async move {
            if let Some(identity) = fut.await?.identity() {
                let user: SlimUser = serde_json::from_str(&identity)?;
                return Ok(user);
            };
            Err(error::ErrorUnauthorized("Unauthorized"))
        })
    }
}
