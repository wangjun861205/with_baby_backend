pub mod eating;
pub mod playing;
pub mod upload;
pub mod user;

use anyhow;
use thiserror;

use crate::domain::user::{PasswordHasher, SaltGenerator};
use actix_web::ResponseError;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use r2d2;
use serde::Serialize;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub static JWT_TOKEN: &str = "JWT_TOKEN";

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    AnyhowError(#[from] anyhow::Error),
    R2D2Error(#[from] r2d2::Error),
}

impl ResponseError for Error {}

pub trait Tokener {
    fn generate(&self, uid: i32) -> Result<String, anyhow::Error>;
    fn validate(&self, token: &str) -> Result<i32, anyhow::Error>;
}

#[derive(Debug, Clone)]
pub struct RandomGenerator {}

impl RandomGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

impl SaltGenerator for RandomGenerator {
    fn gen(&self) -> String {
        return "fake".into();
    }
}

#[derive(Debug, Clone)]
pub struct Hasher {}

impl Hasher {
    pub fn new() -> Self {
        Self {}
    }
}

impl PasswordHasher for Hasher {
    fn hash(&self, salt: &str, password: &str) -> String {
        return "fake password".into();
    }
}

#[derive(Debug, Serialize)]
pub struct QueryResponse<T: Serialize> {
    list: Vec<T>,
    total: i64,
}
