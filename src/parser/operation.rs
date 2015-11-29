#![allow(non_snake_case)]

use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use super::error::ParseError;

#[derive(Deserialize, Debug)]
pub struct Operation {
    pub name: String,
    pub http: HTTP,
    pub input: Input,
    pub output: Option<Output>,
    pub errors: Vec<Error>,
    pub deprecated: Option<bool>, // Silliness for Serde
    pub documentation: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct HTTP {
    method: String,  // Would be nice to make this an enum...
    requestUri: String,
    responseCode: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct Error {
    shape: String,
    error: HTTPError,
    exception: bool, // Silliness -- this should be ignored
    documentation: Option<String>,
    deprecated: Option<bool>, // Silliness, false by default
}

#[derive(Deserialize, Debug)]
pub struct HTTPError {
    httpStatusCode: i32
}

#[derive(Deserialize, Debug)]
pub struct Output {
    shape: String,
    documentation: Option<String>,
    deprecated: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct Input {
    shape: String,
    deprecated: Option<bool>, // Silly, false by default
}

