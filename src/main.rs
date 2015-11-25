#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![allow(non_snake_case)]
#![allow(unused_attributes)]

extern crate serde;
extern crate serde_json;

use std::fs::File;
use parser::ServiceDefinition;

mod parser;
mod testhelpers;

const SERVICE_FILE: &'static str = "fixtures/services/lambda-2015-03-31.json";

#[allow(unused_mut)]
fn main() {
    let mut fd = File::open(SERVICE_FILE).unwrap();
    let lambda_definition: ServiceDefinition = serde_json::from_reader(fd).unwrap();
    println!("{:?}", lambda_definition);
}
