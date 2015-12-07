#![feature(custom_derive, plugin, convert)]
#![plugin(serde_macros)]
#![allow(non_snake_case)]

extern crate serde;
extern crate serde_json;

use std::fs::File;
use self::parser::ServiceDefinition;

pub mod parser;
pub mod generater;

#[cfg(test)]
mod testhelpers;

fn usage() {
  println!("usage: awsparser [input] [output]\n\
            - [input] is a json service definition from the python aws sdk\n\
            - [output] is a rust file the generated code should be written to.\n\
            \toutput must not exist, but its directory must already exist.\n\
            \n\
            Ex:\n\
            \tawsparser lambda-service.json lambda.rs\n\
          ");
}

fn main() {
    // Parse command line arguments
    let mut args = std::env::args();
    if args.len() < 3 {
        return usage();
    }

    // File IO
    let path = args.nth(1).expect("Input parameter missing");
    println!("Input path '{}'", &path);
    let input = File::open(&path).expect(format!("AWS service definition {} cannot be opened. Check path and permissions", &path).as_str());
    let path = args.next().expect("Output parameter missing");
    println!("Output path '{}'", &path);
    let mut output = File::create(&path).expect(format!("Rust output file {} cannot be opened. Check path and permissions", &path).as_str());

    // Parse json
    let definition = match ServiceDefinition::parse(input) {
        Ok(definition) => definition,
        Err(error) => panic!("Error parsing {}: {:?}", &path, error),
    };

    // Print to stdout
    if let Err(error) = definition.generate(&mut output) {
        println!("Error generating code: {:?}", error);
    }
}
