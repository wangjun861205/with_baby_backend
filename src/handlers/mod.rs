pub(crate) mod location;
pub(crate) mod memory;
pub(crate) mod models;
pub(crate) mod upload;
pub(crate) mod user;

use anyhow;

use crate::domain::user::{PasswordHasher, SaltGenerator};
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use serde::Serialize;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub static JWT_TOKEN: &str = "JWT_TOKEN";

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

impl<T: Serialize> QueryResponse<T> {
    pub fn new(list: Vec<T>, total: i64) -> Self {
        Self { list: list, total: total }
    }
}
