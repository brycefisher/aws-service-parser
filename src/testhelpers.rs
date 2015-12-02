extern crate serde;
extern crate serde_json;

use serde_json::Value;
use std::fs::File;
use std::collections::BTreeMap;
use std::io::Read;

/// Returns a File for a given path (relative to project src/fixtures dir).
/// This method panics on error, which is exactly should happen in tests.
/// The returned File object is a Reader.
/// Ex: to open the fixture fixtures/services/foo.json do `fixture_reader("services/foo")`
pub fn fixture_reader(path: &str) -> File {
    File::open(format!("fixtures/{}.json", path)).unwrap()
}

#[allow(unused_mut)]
/// Returns a serde_json Value from a fixture or panics.
pub fn fixture_json(path: &str) -> Value {
    let mut fd = fixture_reader(path);
    serde_json::from_reader(fd).unwrap()
}

/// Returns a BTreeMap from a json fixture or panics.
pub fn fixture_btreemap(path: &str) -> BTreeMap<String, Value> {
    let json = fixture_json(path);
    json.as_object().unwrap().clone()
}

/// Returns a portion of the integer fixture file or panics.
pub fn fixture_integer(fixture: &str) -> BTreeMap<String, Value> {
    fixture_btreemap("shape-types/integers")
        .get(fixture).expect("That fixture is not inside fixtures/shape-types/integers.json")
        .as_object().unwrap()
        .clone()
}

pub fn fixture_string(path: &str) -> String {
    let mut fd = File::open(format!("fixtures/{}", path)).unwrap();
    let mut fixture = String::new();
    fd.read_to_string(&mut fixture).unwrap();
    fixture
}
