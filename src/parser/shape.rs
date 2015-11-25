extern crate serde;
extern crate serde_json;

use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Shape {
    shape_type: ShapeType,
    name: String,
}

impl ShapeType {
    pub fn from_serde_json(obj: &BTreeMap<String, Value>) -> Result<ShapeType, ParseError> {
        let shape_type = match obj.get("type") {
            Some(&Value::String(ref s)) => s.as_bytes(),
            _ => return Err(ParseError::TypeStringMissing)
        };
        match shape_type {
            b"boolean" => Ok(ShapeType::Boolean),
            b"double" => Ok(ShapeType::Double),
            b"float" => Ok(ShapeType::Float),
            b"long" => Ok(ShapeType::Long),
            b"timestamp" => Ok(ShapeType::Timestamp),
            b"blob" |
            b"integer" |
            b"string" |
            b"structure" => Err(ParseError::NotImplemented),
            _ => Err(ParseError::InvalidTypeString)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    ExpectedObject,
    TypeStringMissing,
    NotImplemented,
    InvalidTypeString
}

#[derive(Debug, PartialEq)]
pub enum ShapeType {
    Blob(bool),                     // Vec<u8>, true if streaming
    Boolean,                        // bool
    Double,                         // f64
    Float,                          // f32
    Integer,                        // i32
    IntegerRange(IntegerRange),     // i32 with bounds
    // List,                        // TODO figure out how to implement this...
    Long,                           // i64
    StringEnum(Vec<String>),        // Vec<String> of enum variants
    StringPattern(StringPattern),   // regex, plus optional lengths
    Structure(Structure),           // custom struct
    Exception(Exception),           // custom struct
    Timestamp,                      // TODO - determine Rust type for this
}

#[derive(Debug, PartialEq)]
pub struct IntegerRange {
    pub min: i32,
    pub max: i32
}

#[derive(Debug, PartialEq)]
pub struct StringPattern {
    pub pattern: Option<String>, // TODO - use regex??
    pub min: Option<i32>,
    pub max: Option<i32>,
}

#[derive(Debug, PartialEq)]
pub struct Structure {
    pub required: Option<Vec<String>>,
    pub members: Vec<Member>,
}

#[derive(Debug, PartialEq)]
pub struct Exception {
    pub required: Option<Vec<String>>,
    pub members: Vec<Member>,
    pub status_code: i32, // TODO use hyper status codes instead
    pub documentation: String,
}

#[derive(Debug, PartialEq)]
pub struct Member {
    pub shape: String, // TODO try to make this a Box<Shape>
    pub documentation: String,
}

#[cfg(test)]
mod test {
    extern crate serde;
    extern crate serde_json;

    use serde_json::Value;
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn parse_boolean_shape_from_serde_json_value() {
        let mut input = BTreeMap::new();
        input.insert("type".to_string(), Value::String("boolean".to_string()));
        let output = ShapeType::from_serde_json(&input);
        assert_eq!(output, Ok(ShapeType::Boolean));
    }

    #[test]
    fn parse_double_shape_from_serde_json_value() {
        let mut input = BTreeMap::new();
        input.insert("type".to_string(), Value::String("double".to_string()));
        let output = ShapeType::from_serde_json(&input);
        assert_eq!(output, Ok(ShapeType::Double));
    }

    #[test]
    fn parse_float_shape_from_serde_json_value() {
        let mut input = BTreeMap::new();
        input.insert("type".to_string(), Value::String("float".to_string()));
        let output = ShapeType::from_serde_json(&input);
        assert_eq!(output, Ok(ShapeType::Float));
    }

    #[test]
    fn parse_long_shape_from_serde_json_value() {
        let mut input = BTreeMap::new();
        input.insert("type".to_string(), Value::String("long".to_string()));
        let output = ShapeType::from_serde_json(&input);
        assert_eq!(output, Ok(ShapeType::Long));
    }

    #[test]
    fn parse_timestamp_shape_from_serde_json_value() {
        let mut input = BTreeMap::new();
        input.insert("type".to_string(), Value::String("timestamp".to_string()));
        let output = ShapeType::from_serde_json(&input);
        assert_eq!(output, Ok(ShapeType::Timestamp));
    }

    #[test]
    fn parse_error_invalid_shape_type() {
        let mut input = BTreeMap::new();
        input.insert("type".to_string(), Value::String("invalid-type".to_string()));
        let output = ShapeType::from_serde_json(&input);
        assert_eq!(output, Err(ParseError::InvalidTypeString));
    }
}
