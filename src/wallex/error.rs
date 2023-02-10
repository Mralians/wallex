#![allow(dead_code)]

use http::status::InvalidStatusCode;
use http::StatusCode;
use std::convert::From;
use thiserror::Error;

#[derive(Debug)]
pub struct Error {
    kind:Kind,
}
#[derive(Debug)]
enum Kind {
    Json(serde_json::error::Error),
    Http(reqwest::Error),
    Lib(String),
}

impl From<serde_json::error::Error> for  Error {
    fn from(err: serde_json::error::Error) -> Self {
        Error {
            kind: Kind::Json(err),
        }
    }
}
impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error {
            kind: Kind::Http(err),
        }
    }
}

pub(crate) fn lib(err: impl Into<String>) -> Error {
    Error {
        kind: Kind::Lib(err.into()),
    }
}
impl  Error {
    pub fn status_code(&self) -> Option<StatusCode> {
        match &self.kind {
            Kind::Http(err) => err.status(),
            _ => None,
        }
    }
    pub fn is_timeout(&self) -> bool {
        match &self.kind {
            Kind::Http(err) => err.is_timeout(),
            _ => false,
        }
    }
    pub fn is_json(self) -> bool {
        match &self.kind {
            Kind::Json(_) => true,
            _ => false,
        }
    }
}

use std::error;
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            Kind::Lib(_) => None,
            Kind::Http(err) => Some(err),
            Kind::Json(err) => Some(err),
        }
    }
}
use std::fmt;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            Kind::Lib(err) => err.fmt(f),
            Kind::Http(err) => err.fmt(f),
            Kind::Json(err) => err.fmt(f),
        }
    }
}
