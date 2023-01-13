#![allow(dead_code)]

use serde::{Deserialize, Deserializer};

// Wallex Responce
#[derive(Debug, Clone, Deserialize)]
pub struct WallexResp<T>
where
    for<'a> T: Deserialize<'a>,
{
    #[serde(deserialize_with = "object_empty_as_none")]
    result: Option<T>,
    #[serde(default = "default_resource")]
    code: u16,
    message: String,
    success: bool,
}
fn object_empty_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    for<'a> T: Deserialize<'a>,
{
    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct Empty {}

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum Aux<T> {
        T(T),
        Empty(Empty),
        Null,
    }

    match Deserialize::deserialize(deserializer)? {
        Aux::T(t) => Ok(Some(t)),
        Aux::Empty(_) | Aux::Null => Ok(None),
    }
}
#[inline]
fn default_resource() -> u16 {
    200
}

impl<T> WallexResp<T>
where
    for<'a> T: Deserialize<'a>,
{
    pub fn status_code(&self) -> u16 {
        self.code
    }
    pub fn result<'a>(&'a self) -> &'a Option<T> {
        &self.result
    }
    pub fn message(&self) -> String {
        self.message.clone()
    }
    pub fn success(self) -> bool {
        self.success
    }
}


