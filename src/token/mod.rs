pub mod jwt;

use crate::error::Error;
use actix_web::{dev::Payload, FromRequest, HttpMessage, HttpRequest};
use std::future::{ready, Future};
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct UID(pub i32);

impl FromRequest for UID {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<UID, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(&UID(uid)) = req.extensions().get::<Self>() {
            return Box::pin(async move { Ok(UID(uid)) });
        }
        return Box::pin(ready(Err(Error::PermissionError)));
    }
}
