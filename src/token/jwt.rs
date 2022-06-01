use crate::handlers;
use crate::handlers::Tokener;
use crate::token::UID;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use anyhow::{self, Context};
use chrono::Duration;
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

#[derive(Debug, Clone)]
pub struct JWT {
    secret: String,
    duration: Duration,
}

impl JWT {
    pub fn new(secret: &str, duration: Duration) -> Self {
        Self {
            secret: secret.to_owned(),
            duration: duration,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    exp: usize,
    uid: i32,
}

impl Tokener for JWT {
    fn generate(&self, uid: i32) -> Result<String, anyhow::Error> {
        let s = encode(
            &Header::default(),
            &Claims {
                exp: (chrono::Local::now() + self.duration).timestamp() as usize,
                uid: uid,
            },
            &EncodingKey::from_secret(&self.secret.as_bytes()),
        )?;
        Ok(s)
    }
    fn validate(&self, token: &str) -> Result<i32, anyhow::Error> {
        let TokenData { header, claims } =
            decode::<Claims>(token, &DecodingKey::from_secret(&self.secret.as_bytes()), &Validation::new(jsonwebtoken::Algorithm::HS256)).context("failed to valid jwt token")?;
        if claims.exp < chrono::Local::now().timestamp() as usize {
            return Err(anyhow::Error::msg("expired token").context("failed to valid jwt token"));
        }
        Ok(claims.uid)
    }
}

impl<S, B> Transform<S, ServiceRequest> for JWT
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JWTMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JWTMiddleware { service: service, jwt: self.clone() }))
    }
}

pub struct JWTMiddleware<S> {
    service: S,
    jwt: JWT,
}

impl<S, B> Service<ServiceRequest> for JWTMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers().get(handlers::JWT_TOKEN);
        if headers.is_none() {
            return Box::pin(ready(Err(handlers::Error::from(anyhow::Error::msg("failed to extract jwt token from request headers")).into())));
        }
        let token = headers.unwrap().to_str();
        if let Ok(t) = token {
            match self.jwt.validate(t) {
                Err(err) => {
                    return Box::pin(async move { Err(Error::from(crate::handlers::Error(err.context("failed to valid jwt token")))) });
                }
                Ok(uid) => {
                    req.extensions_mut().insert(UID(uid));
                    let fut = self.service.call(req);
                    return Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    });
                }
            }
        }
        return Box::pin(ready(Err(handlers::Error::from(anyhow::Error::msg("invalid jwt token")).into())));
    }
}
