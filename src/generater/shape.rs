use std::io::Error;
use std::io::prelude::*;
use ::parser::*;

impl Shape {
    pub fn generate<W: Write>(&self, out: &mut W) -> Result<(), Error> {
        let shape_type = &self.shape_type;
        let rust_type = match shape_type {
            &ShapeType::Blob(_) => "Vec<u8>".to_string(), // TODO -- use streaming bool...
            &ShapeType::Boolean => "bool".to_string(),
            &ShapeType::Double => "f64".to_string(),
            &ShapeType::Float => "f32".to_string(),
            &ShapeType::Integer(_) => "i32".to_string(), // TODO -- use min/max info...
            &ShapeType::List(List(ref list_type)) => format!("Vec<{}>", &list_type.to_string()),
            &ShapeType::Long => "i64".to_string(),
            &ShapeType::StringEnum(ref string_enum) => return string_enum.generate(out, &self.name),
            &ShapeType::Timestamp |
            &ShapeType::StringPattern(_) => "String".to_string(),
            &ShapeType::Structure(ref structure) => return structure.generate(out, &self.name),
            &ShapeType::Exception(ref exception) => return exception.generate(out, &self.name),
        };
        try!(writeln!(out, "pub type {} = {};", &self.name, rust_type));
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

impl StringEnum {

    /// This method is a bit peculiar in that it hijacks `generate()` entirely.
    /// That's because enums are not allowed to take the form `pub type MyEnum = ...;`
    /// (which most other shapes follow). Instead it must take the form:
    /// `pub enum MyEnum { ... }`. This keeps the implementation clearer for all
    /// the normal cases in generate.
    pub fn generate<W: Write>(&self, out: &mut W, name: &str) -> Result<(), Error> {
        try!(writeln!(out, "pub enum {} {{", name));
        for variant in &self.0 {
            try!(writeln!(out, "    {},", variant));
        }
        try!(writeln!(out, "}};"));
        Ok(())
    }
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

impl Exception {
    pub fn generate<W: Write>(&self, out: &mut W, name: &str) -> Result<(), Error> {
        try!(writeln!(out, "#[derive(Debug)]"));
        if let Some(ref docs) = self.documentation {
          try!(writeln!(out, "/// {}", docs));
        }
        try!(writeln!(out, "pub struct {} {{", name));
        for member in &self.members {
            member.generate(out);
        }
        try!(writeln!(out, "}}\n"));

        try!(writeln!(out, "impl ::std::error::Error for {} {{", name));
        try!(writeln!(out, "    pub fn description(&self) -> &str {{"));
        try!(write!(out, "        &format!(\"{}:", name));
        for _ in &self.members {
            try!(write!(out, " {{}}"));
        }
        try!(write!(out, "\""));
        for member in &self.members {
            try!(write!(out, ", self.{}", member.name));
        }
        try!(writeln!(out, ");"));
        try!(writeln!(out, "    }}\n"));
        try!(writeln!(out, "    pub fn cause(&self) -> Option<&Error> {{"));
        try!(writeln!(out, "        None"));
        try!(writeln!(out, "    }}"));
        try!(writeln!(out, "}}"));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::parser::*;
    use ::testhelpers::fixture_string;
    use std::io::Write;

    macro_rules! generates {
        ($test:ident, $fixture:expr, $input:expr) => {
            #[test]
            fn $test() {
                let input = $input;
                let mut buffer = Vec::new();
                assert!(input.generate(&mut buffer).is_ok());
                let actual = String::from_utf8(buffer).unwrap();
                let expected = fixture_string(&format!("generated/{}.rs", $fixture));
                assert_eq!(expected, actual);
            }
        };
    }

    generates!(boolean, "boolean", Shape {
        name: "Enabled".to_string(),
        shape_type: ShapeType::Boolean,
    });

    generates!(double, "double-trouble", Shape {
        name: "Trouble".to_string(),
        shape_type: ShapeType::Double,
    });

    generates!(list, "list", Shape {
        name: "AllTheThings".to_string(),
        shape_type: ShapeType::List(List("Thing".to_string())),
    });

    generates!(string_enum, "string_enum", Shape {
        name: "WhereIsCarmenSanDiego".to_string(),
        shape_type: ShapeType::StringEnum(StringEnum(vec![
            "Berlin".to_string(),
            "Madrid".to_string(),
            "Toronto".to_string(),
            "Beijing".to_string(),
        ])),
    });

    generates!(structure, "structure-genie-in-a-bottle", Shape {
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

    generates!(string_pattern, "string_pattern", Shape {
        name: "AsciiArt".to_string(),
        shape_type: ShapeType::StringPattern(StringPattern {
            pattern: ".*".to_string(),
            min: None,
            max: None,
        }),
    });

    generates!(exception, "exception", Shape {
        name: "ServiceException".to_string(),
        shape_type: ShapeType::Exception(Exception {
            documentation: Some("The AWS Lambda service encountered an internal error.".to_string()),
            status_code: 500,
            members: vec![
                Member {
                    name: "Type".to_string(),
                    required: true,
                    documentation: None,
                    shape: "String".to_string(),
                    location: Location::Body,
                },
                Member {
                    name: "Message".to_string(),
                    required: true,
                    documentation: None,
                    shape: "String".to_string(),
                    location: Location::Body,
                },
            ],
        }),
    });
}
