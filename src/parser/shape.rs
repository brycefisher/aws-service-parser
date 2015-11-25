extern crate serde;
extern crate serde_json;

use serde_json::Value;
use super::shape_type::ShapeType;
use super::error::ParseError;

#[derive(Debug, PartialEq)]
pub struct Shape {
    shape_type: ShapeType,
    name: String,
}

impl Shape {
    fn parse(_name: String, json: Value) -> Result<Shape, ParseError> {
        Err(ParseError::NotImplemented)
    }
}

#[cfg(test)]
mod test {
    extern crate serde;
    extern crate serde_json;

    use super::*;
    use ::testhelpers::fixture_btreemap;
}
