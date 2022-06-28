use actix_web::{http::StatusCode, ResponseError};
use thiserror;

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub enum Error {
    #[error("no permission")]
    PermissionError,
    #[error("{}", .0)]
    BusinessError(String),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            &Self::PermissionError => StatusCode::FORBIDDEN,
            &Self::BusinessError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
