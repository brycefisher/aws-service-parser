extern crate serde;
extern crate serde_json;

#[derive(Deserialize,Debug)]
/// Stores the totality of a service definition json file.
pub struct ServiceDefinition {
    version: f64,
    documentation: String,
    metadata: Metadata,
    operations: serde_json::Value,
    shapes: serde_json::Value,
    examples: serde_json::Value,
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

#[cfg(test)]
mod test {
    extern crate serde;
    extern crate serde_json;

    use super::*;
    use std::fs::File;

    #[test]
    #[allow(unused_mut)]
    fn parse_service_definition_without_error() {
        let mut fd = File::open("fixtures/services/lambda-2015-03-31.json").unwrap();
        let _: ServiceDefinition = serde_json::from_reader(fd).unwrap();
    }
}
