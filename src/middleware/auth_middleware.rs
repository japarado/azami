use crate::models::user::AuthUser;
use actix_identity::Identity;
use actix_web::{dev::Payload, error, FromRequest, HttpRequest, HttpResponse};
use futures::future::Future;
use std::pin::Pin;

// pub type SlimUser = SlimUser;

impl FromRequest for AuthUser {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<AuthUser, Self::Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let fut = Identity::from_request(req, pl);

        Box::pin(async move {
            if let Some(identity) = fut.await?.identity() {
                println!("{}", identity);
                let user: AuthUser = serde_json::from_str(&identity)?;
                return Ok(user);
            };
            Err(error::ErrorUnauthorized("Unauthorized"))
        })
    }
}

