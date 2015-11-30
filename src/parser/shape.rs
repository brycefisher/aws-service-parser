extern crate serde;
extern crate serde_json;

use serde_json::Value;
use super::shape_type::ShapeType;
use super::error::ParseError;

#[derive(Debug, PartialEq)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub name: String,
}

impl Shape {
    // TODO learn to use lifetimes here to avoid allocations
    pub fn parse(name: &str, json: &Value) -> Result<Shape, ParseError> {
        let obj = match json.as_object() {
            Some(obj) => obj,
            None => return Err(ParseError::ExpectedObject)
        };
        let shape_type = match ShapeType::parse(obj) {
            Ok(shape_type) => shape_type,
            Err(ParseError::InvalidMember(_)) => return Err(ParseError::InvalidMember(name.to_string())),
            Err(err) => return Err(err),
        };
        Ok(Shape {
            name: name.to_string(),
            shape_type: shape_type
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::shape_type::*;
    use ::testhelpers::fixture_btreemap;

    fn primitive_shape(name: &str) -> serde_json::Value {
        fixture_btreemap("shapes/primitive-shapes").get(name).unwrap().clone()
    }

    #[test]
    fn boolean() {
        let output = Shape::parse("Boolean", &primitive_shape("Boolean"));
        assert_eq!(output, Ok(Shape {
            name:"Boolean".to_string(),
            shape_type: ShapeType::Boolean,
        }));
    }

    #[test]
    fn double() {
        let output = Shape::parse("Double", &primitive_shape("Double"));
        assert_eq!(output, Ok(Shape {
            name:"Double".to_string(),
            shape_type: ShapeType::Double,
        }));
    }

    #[test]
    fn timestamp() {
        let output = Shape::parse("Date", &primitive_shape("Date"));
        assert_eq!(output, Ok(Shape {
            name:"Date".to_string(),
            shape_type: ShapeType::Timestamp,
        }));
    }

    #[test]
    fn long() {
        let output = Shape::parse("Long", &primitive_shape("Long"));
        assert_eq!(output, Ok(Shape {
            name:"Long".to_string(),
            shape_type: ShapeType::Long,
        }));
    }

    #[test]
    fn float() {
        let output = Shape::parse("Float", &primitive_shape("Float"));
        assert_eq!(output, Ok(Shape {
            name:"Float".to_string(),
            shape_type: ShapeType::Float,
        }));
    }
}
