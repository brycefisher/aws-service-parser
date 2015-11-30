use std::io::prelude::*;
use ::parser::shape::Shape;
use ::parser::shape_type::{ShapeType, List};
use super::error::*;

impl Shape {
    pub fn generate<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        let shape_type = &self.shape_type;
        let rust_type = match shape_type {
            &ShapeType::Blob(_) => "Vec<u8>".to_string(), // TODO -- use streaming bool...
            &ShapeType::Boolean => "bool".to_string(),
            &ShapeType::Double => "f64".to_string(),
            &ShapeType::Float => "f32".to_string(),
            &ShapeType::Integer(_) => "i32".to_string(), // TODO -- use min/max info...
            &ShapeType::List(List(ref list_type)) => {
                format!("Vec<{}>", &list_type.to_string())
            },
            &ShapeType::Long => "i64".to_string(),
            //StringEnum(variants) => return StringEnum::to_rust(variants),
            _ => return Err(Error),
        };
        writeln!(out, "pub type {} = {};", &self.name, rust_type);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::parser::{Shape, ShapeType, List};
    use std::io::Write;

    #[test]
    fn boolean() {
        let input = Shape {
            name: "Enabled".to_string(),
            shape_type: ShapeType::Boolean,
        };
        let mut buffer = Vec::new();
        assert!(input.generate(&mut buffer).is_ok());

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "pub type Enabled = bool;\n".to_string());
    }

    #[test]
    fn double() {
        let input = Shape {
            name: "Trouble".to_string(),
            shape_type: ShapeType::Double,
        };
        let mut buffer = Vec::new();
        assert!(input.generate(&mut buffer).is_ok());

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "pub type Trouble = f64;\n".to_string());
    }

    #[test]
    fn list() {
        let input = Shape {
            name: "AllTheThings".to_string(),
            shape_type: ShapeType::List(List("Thing".to_string())),
        };
        let mut buffer = Vec::new();
        assert!(input.generate(&mut buffer).is_ok());

        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "pub type AllTheThings = Vec<Thing>;\n".to_string());
    }
}
