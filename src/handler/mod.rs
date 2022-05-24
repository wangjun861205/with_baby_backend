pub mod user;

use anyhow;
use thiserror;

use crate::domain::user::{PasswordHasher, SaltGenerator};
use actix_web::ResponseError;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] pub anyhow::Error);

impl ResponseError for Error {}

pub trait Tokener {
    fn generate(&self, uid: i32) -> Result<String, anyhow::Error>;
    fn validate(&self, token: &str) -> Result<(), anyhow::Error>;
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
