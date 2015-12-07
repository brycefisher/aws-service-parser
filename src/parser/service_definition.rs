extern crate serde;
extern crate serde_json;

use std::io::Read;
use std::collections::BTreeMap;
use super::error::ParseError;
use super::shape::Shape;
use super::operation::Operation;
use serde_json::Value;

#[derive(Deserialize,Debug)]
/// Intermediate representation of service definition as parsed by serde_json.
struct PartialServiceDefinition {
    version: f64,
    documentation: String,
    metadata: Metadata,
    operations: BTreeMap<String, Operation>,
    shapes: Value,
    examples: Value,
}

#[derive(Debug)]
pub struct ServiceDefinition {
    pub version: f64,
    pub documentation: String,
    pub metadata: Metadata,
    pub operations: Vec<String>,
    pub shapes: Vec<Shape>,
}

#[derive(Deserialize,Debug)]
/// Metadata about this service.
pub struct Metadata {
    apiVersion: String, // Convert to date object
    endpointPrefix: String,
    serviceFullName: String,
    signatureVersion: String, // TODO enum
    protocol: String, // TODO enum
}

impl ServiceDefinition {
    pub fn parse<R: Read>(fd: R) -> Result<ServiceDefinition, ParseError> {
        let partial: PartialServiceDefinition = match serde_json::from_reader(fd) {
            Ok(p) => p,
            Err(_) => return Err(ParseError::SerdeError),

        };
        let obj = try!(partial.shapes.as_object().ok_or(ParseError::ServiceDefinitionInvalidShapes));
        let shapes = try!(ServiceDefinition::parse_shapes(&obj));
        Ok(ServiceDefinition {
            version: partial.version,
            documentation: partial.documentation,
            metadata: partial.metadata,
            operations: vec!(), // TODO implement me!
            shapes: shapes,
        })
    }

    pub fn parse_shapes(obj: &BTreeMap<String, Value>) -> Result<Vec<Shape>, ParseError> {
        let mut shapes = vec!();
        for (key, value) in obj.iter() {
            let shape = try!(Shape::parse(key, value));
            shapes.push(shape);
        }
        Ok(shapes)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::PartialServiceDefinition;
    use ::testhelpers::fixture_reader;

    #[test]
    #[allow(unused_mut)]
    fn partial() {
        let mut fd = fixture_reader("services/lambda-2015-03-31");
        let _: PartialServiceDefinition = serde_json::from_reader(fd).unwrap();
    }
}
