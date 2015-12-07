use std::io::Error;
use std::io::prelude::*;
use ::parser::ServiceDefinition;

impl ServiceDefinition {
    pub fn generate<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        for shape in &self.shapes {
            try!(shape.generate(out));
        }
        Ok(())
    }
}
