mod user;

use anyhow;
use thiserror;

use actix_web::{body::BoxBody, HttpResponse, ResponseError};

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
struct Error(#[from] anyhow::Error);

impl ResponseError for Error {}

pub trait Tokener {
    fn generate(uid: i32) -> String;
    fn valid(token: &str) -> bool;
}
