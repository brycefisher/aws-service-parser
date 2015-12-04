#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![allow(non_snake_case)]

extern crate serde;
extern crate serde_json;

use std::io::{self, Write};

mod parser;
mod generater;

#[cfg(test)]
mod testhelpers;

fn main() {
    // Parse command line arguments
    let args = std::env::args();
    if args.is_none() || args.count() < 2 {
        panic!("must pass in the json file path to a service you wish to parse");
    }

    // File IO
    let path = args.nth(1).unwrap();
    let fd = std::fs::File(&path).expect(format!("File {} cannot be opened. Check path and permissions", &path));

    // Parse json
    let definition = match self::parse::ServiceDefinition::parse(fd) {
        Ok(definition) => definition,
        Err(error) => panic!("Error parsing {}: {:?}", &path, error),
    };

    // Print to stdout
    if let Err(error) = definition.generate(io::stdout()) {
        panic!("Error generating code: {:?}", error);
    }
}
