#![allow(dead_code)]

use http::status::InvalidStatusCode;
use http::StatusCode;
use std::convert::From;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WallexError {
    #[error("reqwest error")]
    ReqwestErr(String),
    #[error("invalid status code")]
    InvalidStatusCodeErr(String),
    #[error("bad request")]
    BadRequestErr,
    #[error("unauthorized")]
    UnauthorizedErr,
    #[error("access forbidden")]
    ForbiddenErr,
    #[error("resource not found")]
    NotFoundErr,
    #[error("unknown error")]
    UnknownErr,
}

impl From<reqwest::Error> for WallexError {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestErr(e.to_string())
    }
}
impl From<InvalidStatusCode> for WallexError {
    fn from(e: InvalidStatusCode) -> Self {
        Self::InvalidStatusCodeErr(e.to_string())
    }
}
pub fn err_non_ok_response(code: StatusCode) -> Result<(), WallexError> {
    match code {
        StatusCode::NOT_FOUND => Err(WallexError::NotFoundErr),
        StatusCode::BAD_REQUEST => Err(WallexError::BadRequestErr),
        StatusCode::UNAUTHORIZED => Err(WallexError::UnauthorizedErr),
        StatusCode::FORBIDDEN => Err(WallexError::ForbiddenErr),
        _ => Err(WallexError::UnknownErr),
    }
}
