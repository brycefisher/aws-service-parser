use std::io::Error;
use std::io::prelude::*;
use ::parser::shape::Shape;
use ::parser::shape_type::{ShapeType, List, StringEnum, Member, Location, Structure};

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
            &ShapeType::StringEnum(StringEnum(ref variants)) => return generate_string_enum(out, &self.name, variants),
            &ShapeType::Structure(ref structure) => return structure.generate(out, &self.name),
            _ => unimplemented!()
        };
        writeln!(out, "pub type {} = {};", &self.name, rust_type);
        Ok(())
    }
}

impl Member {
    pub fn generate<W:Write>(&self, out: &mut W) -> Result<(), Error> {
        if let Some(ref documentation) = self.documentation {
            try!(writeln!(out, "    /// {}", documentation));
        }
        let name = &self.name;
        let shape = &self.shape;
        match self.required {
            true => try!(writeln!(out, "    pub {name}: {shape},", name=name, shape=shape)),
            false => try!(writeln!(out, "    pub {name}: Option<{shape}>,", name=name, shape=shape)),
        };
        Ok(())
    }
}

/// This method is a bit peculiar in that it hijacks `generate()` entirely.
/// That's because enums are not allowed to take the form `pub type MyEnum = ...;`
/// (which all the other shapes follow). Instead it must take the form:
/// `pub enum MyEnum { ... }`. This keeps the implementation clearer for all
/// the normal cases in generate.
fn generate_string_enum<W: Write>(out: &mut W, name: &str, variants: &Vec<String>) -> Result<(), Error> {
    // TODO: propagate errors from `writeln!`
    writeln!(out, "pub enum {} {{", name);
    for variant in variants {
        writeln!(out, "    {},", variant);
    }
    writeln!(out, "}};");
    Ok(())
}

impl Structure {
    pub fn generate<W: Write>(&self, out: &mut W, name: &str) -> Result<(), Error> {
        try!(writeln!(out, "#[derive(Debug, Default)]"));
        try!(writeln!(out, "pub struct {} {{", name));
        for member in &self.0 {
            member.generate(out);
        }
        try!(writeln!(out, "}}"));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::parser::{Shape, ShapeType, List, StringEnum, Member, Location, Structure};
    use std::io::Write;

    macro_rules! generates {
        ($test:ident, $output:expr, $input:expr) => {
            #[test]
            fn $test() {
                let input = $input;
                let mut buffer = Vec::new();
                assert!(input.generate(&mut buffer).is_ok());
                let output = String::from_utf8(buffer).unwrap();
                assert_eq!(output, $output.to_string());
            }
        };
    }

    generates!(boolean, "pub type Enabled = bool;\n", Shape {
        name: "Enabled".to_string(),
        shape_type: ShapeType::Boolean,
    });

    generates!(double, "pub type Trouble = f64;\n", Shape {
        name: "Trouble".to_string(),
        shape_type: ShapeType::Double,
    });

    generates!(list, "pub type AllTheThings = Vec<Thing>;\n", Shape {
        name: "AllTheThings".to_string(),
        shape_type: ShapeType::List(List("Thing".to_string())),
    });

    generates!(string_enum, "pub enum WhereIsCarmenSanDiego {\n    Berlin,\n    Madrid,\n    Toronto,\n    Beijing,\n};\n", Shape {
        name: "WhereIsCarmenSanDiego".to_string(),
        shape_type: ShapeType::StringEnum(StringEnum(vec![
            "Berlin".to_string(),
            "Madrid".to_string(),
            "Toronto".to_string(),
            "Beijing".to_string(),
        ])),
    });

    generates!(structure, "#[derive(Debug, Default)]\npub struct GenieInABottle {\n    pub owner: Option<Person>,\n    pub wishes: integer,\n}\n", Shape {
        name: "GenieInABottle".to_string(),
        shape_type: ShapeType::Structure(Structure(vec![
            Member {
                name: "owner".to_string(),
                shape: "Person".to_string(),
                documentation: None,
                required: false,
                location: Location::Body
            },
            Member {
                name: "wishes".to_string(),
                shape: "integer".to_string(),
                documentation: None,
                required: true,
                location: Location::Body
            },
        ]))
    });
}
