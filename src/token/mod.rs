pub mod jwt;

use crate::handlers;
use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use std::future::{ready, Future};
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct UID(pub i32);

impl FromRequest for UID {
    type Error = handlers::Error;
    type Future = Pin<Box<dyn Future<Output = Result<UID, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(&UID(uid)) = req.extensions().get::<Self>() {
            return Box::pin(async move { Ok(UID(uid)) });
        }
        return Box::pin(ready(Err(handlers::Error::from(anyhow::Error::msg("uid not exists")))));
    }
}
