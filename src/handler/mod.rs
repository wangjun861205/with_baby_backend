mod user;

use anyhow;
use thiserror;

use actix_web::ResponseError;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] pub anyhow::Error);

impl ResponseError for Error {}

pub trait Tokener {
    fn generate(&self, uid: i32) -> Result<String, anyhow::Error>;
    fn valid(&self, token: &str) -> Result<(), anyhow::Error>;
}
