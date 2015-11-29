extern crate serde;
extern crate serde_json;

use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use super::error::ParseError;

#[derive(Debug, PartialEq)]
pub enum ShapeType {
    Blob(Blob),                     // custom struct
    Boolean,                        // bool
    Double,                         // f64
    Float,                          // f32
    Integer(Integer),               // i32
    List(List),                     // custom struct
    Long,                           // i64
    StringEnum(StringEnum),         // custom struct
    StringPattern(StringPattern),   // custom struct
    Structure(Structure),           // custom struct
    Exception(Exception),           // custom struct
    Timestamp,                      // TODO - determine Rust type for this
}

impl ShapeType {
    pub fn parse(obj: &BTreeMap<String, Value>) -> Result<ShapeType, ParseError> {
        let shape_type = match obj.get("type") {
            Some(&Value::String(ref s)) => s.as_bytes(),
            _ => return Err(ParseError::TypeStringMissing)
        };
        match shape_type {
            b"blob" => Blob::parse(obj),
            b"boolean" => Ok(ShapeType::Boolean),
            b"double" => Ok(ShapeType::Double),
            b"float" => Ok(ShapeType::Float),
            b"integer" => Integer::parse(obj),
            b"list" => List::parse(obj),
            b"long" => Ok(ShapeType::Long),
            b"structure" => parse_structure_or_exception(obj),
            b"timestamp" => Ok(ShapeType::Timestamp),
            b"string" => parse_string_enum_or_pattern(obj),
            _ => Err(ParseError::InvalidTypeString)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Blob(bool);

impl Blob {
    pub fn parse(obj: &BTreeMap<String, Value>) -> Result<ShapeType, ParseError> {
        let streaming = obj.contains_key("streaming");
        Ok(ShapeType::Blob(Blob(streaming)))
    }
}

#[derive(Debug, PartialEq)]
pub struct Integer {
    pub min: Option<i64>,
    pub max: Option<i64>
}

impl Integer {
    pub fn parse(obj: &BTreeMap<String, Value>) -> Result<ShapeType, ParseError> {
        let max = match obj.get("max") {
            Some(json) => match json.as_i64() {
                Some(max) => Some(max),
                None => return Err(ParseError::InvalidMaxInteger),
            },
            None => None,
        };
        let min = match obj.get("min") {
            Some(json) => match json.as_i64() {
                Some(min) => Some(min),
                None => return Err(ParseError::InvalidMinInteger),
            },
            None => None,
        };
        Ok(ShapeType::Integer(Integer {
            max: max,
            min: min,
        }))
    }
}

#[derive(Debug, PartialEq)]
pub struct List(String);

impl List {
    pub fn parse(obj: &BTreeMap<String, Value>) -> Result<ShapeType, ParseError> {
        let json = try!(obj.get("member").ok_or(ParseError::MissingListMember));
        let member = try!(json.as_object().ok_or(ParseError::InvalidListMember));
        let json = try!(member.get("shape").ok_or(ParseError::MissingListShape));
        let shape = try!(json.as_string().ok_or(ParseError::InvalidListShape));
        Ok(ShapeType::List(List(shape.to_string())))
    }
}

pub fn parse_string_enum_or_pattern(obj: &BTreeMap<String, Value>) -> Result<ShapeType, ParseError> {
    if obj.contains_key("enum") {
        return StringEnum::parse(obj);
    }
    StringPattern::parse(obj)
}

#[derive(Debug, PartialEq)]
pub struct StringEnum(Vec<String>);

impl StringEnum {
    pub fn parse(obj: &BTreeMap<String, Value>) -> Result<ShapeType, ParseError> {
        // Safe -- will not panic we've checked that `enum` exists prior to this function call.
        let json = obj.get("enum").unwrap();
        let array = try!(json.as_array().ok_or(ParseError::InvalidStringEnum));
        let mut variants: Vec<String> = vec!();
        for json in array {
            let variant = try!(json.as_string().ok_or(ParseError::InvalidStringVariant));
            variants.push(variant.to_string());
        }
        Ok(ShapeType::StringEnum(StringEnum(variants)))
    }
}

#[derive(Debug, PartialEq)]
pub struct StringPattern {
    pub pattern: String, // TODO - use regex?? - default of .*
    pub min: Option<i64>,
    pub max: Option<i64>,
}

impl StringPattern {
    pub fn parse(obj: &BTreeMap<String, Value>) -> Result<ShapeType, ParseError> {
        let max = match obj.get("max") {
            Some(ref json) => {
                let max = try!(json.as_i64().ok_or(ParseError::InvalidStringMax));
                Some(max)
            }
            None => None
        };
        let min = match obj.get("min") {
            Some(ref json) => {
                let min = try!(json.as_i64().ok_or(ParseError::InvalidStringMin));
                // A minimum string length of 0 is effective no minimum.
                if min == 0 {
                    None
                }
                else {
                    Some(min)
                }
            }
            None => None
        };
        let pattern = match obj.get("pattern") {
            Some(json) => try!(json.as_string().ok_or(ParseError::InvalidStringPattern)),
            None => ".*",
        };
        Ok(ShapeType::StringPattern(StringPattern {
            min: min,
            max: max,
            pattern: pattern.to_string(),
        }))
    }
}

pub fn parse_structure_or_exception(obj: &BTreeMap<String, Value>) -> Result<ShapeType, ParseError> {
    if !obj.contains_key("exception") {
        return Structure::parse(obj);
    }
    Exception::parse(obj)
}

#[derive(Debug, PartialEq)]
// TODO - refactor to have members and payload: Option<String> -- validate string is a member name...or do Option<&Member>
pub struct Structure(Vec<Member>);

impl Structure {
    fn parse(obj: &BTreeMap<String, Value>) -> Result<ShapeType, ParseError> {
        // Parse the required fields into a set
        let empty_array = Value::Array(Vec::<Value>::new());
        let json_array = obj.get("required").unwrap_or(&empty_array);
        let empty_vec = Vec::<Value>::new();
        let vec_json = json_array.as_array().unwrap_or(&empty_vec);
        let mut required_members = HashMap::new();
        for json in vec_json {
            let required = try!(json.as_string().ok_or(ParseError::InvalidRequired));
            required_members.insert(required.to_string(), ());
        }

        // Parse member fields into member structs
        let members_value = try!(obj.get("members").ok_or(ParseError::StructureHasNoMembers));
        let raw_members = try!(members_value.as_object().ok_or(ParseError::InvalidStructureMembers));
        let mut members = Vec::new();
        for (name, raw_member) in raw_members.iter() {
            let required = required_members.contains_key(name);
            let member = try!(Member::parse(name, required, raw_member));
            members.push(member);
        }
        Ok(ShapeType::Structure(Structure(members)))
    }
}

#[derive(Debug, PartialEq)]
pub struct Exception {
    pub members: Vec<Member>,
    pub status_code: i64, // TODO use hyper status codes instead
    pub documentation: Option<String>,
}

impl Exception {
    pub fn parse(obj: &BTreeMap<String, Value>) -> Result<ShapeType, ParseError> {
        // Members
        let members = match try!(Structure::parse(obj)) {
            ShapeType::Structure(Structure(m)) => m,
            _ => unreachable!()
        };

        // Optional documentation
        let documentation = obj.get("documentation").map(|d| d.as_string().unwrap().to_string());

        // Status Code
        let json = try!(obj.get("error").ok_or(ParseError::MissingErrorInException));
        let err = try!(json.as_object().ok_or(ParseError::MissingErrorInException));
        let json = try!(err.get("httpStatusCode").ok_or(ParseError::MissingErrorInException));
        let status_code = try!(json.as_i64().ok_or(ParseError::MissingErrorInException));

        Ok(ShapeType::Exception(Exception {
            members: members,
            documentation: documentation,
            status_code: status_code,
        }))
    }
}

#[derive(Debug, PartialEq)]
pub struct Member {
    pub shape: String, // TODO try to make this a Box<Shape>
    pub required: bool,
    pub documentation: Option<String>,
    pub name: String,
    pub location: Location,
}

impl Member {
    fn parse(name: &str, required: bool, raw_member: &Value) -> Result<Member, ParseError> {
        let obj = match raw_member.as_object() {
            Some(o) => o,
            None => return Err(ParseError::InvalidMember(name.to_string()))
        };
        let shape_json = try!(obj.get("shape").ok_or(ParseError::InvalidMember(name.to_string())));
        let shape = try!(shape_json.as_string().ok_or(ParseError::InvalidMember(name.to_string())));
        let documentation = obj.get("documentation").map(|d| d.as_string().unwrap().to_string());
        let location = try!(Location::parse(obj.get("location"), obj.get("locationName")));

        Ok(Member {
            name: name.to_string(),
            required: required,
            documentation: documentation,
            shape: shape.to_string(),
            location: location,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Location {
    Body,
    StatusCode,
    URI(String),
    QueryString(String),
    Header(String),
}

impl Location {
    pub fn parse(location: Option<&Value>, location_name: Option<&Value>) -> Result<Location, ParseError> {
        // By default, location should be Body if not specified (I think...)
        if location.is_none() {
            return Ok(Location::Body);
        }

        // Won't panic because we already checked for None above.
        let json = location.unwrap();
        let location = try!(json.as_string().ok_or(ParseError::InvalidMember("Unknow".to_string())));

        // Status code location doesn't have a locationName.
        if location == "statusCode" {
            return Ok(Location::StatusCode);
        }

        let json = try!(location_name.ok_or(ParseError::InvalidMember("Unknown".to_string())));
        let name = try!(json.as_string().ok_or(ParseError::InvalidMember("Unknown".to_string())));
        match location {
            "uri" => Ok(Location::URI(name.to_string())),
            "querystring" => Ok(Location::QueryString(name.to_string())),
            "header" => Ok(Location::Header(name.to_string())),
            _ => Err(ParseError::NotImplemented),
        }
    }
}

#[cfg(test)]
mod test {
    extern crate serde;
    extern crate serde_json;

    use super::*;
    use super::super::error::ParseError;
    use ::testhelpers::*;

    #[test]
    fn boolean() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/boolean"));
        assert_eq!(output, Ok(ShapeType::Boolean));
    }

    #[test]
    fn double() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/double"));
        assert_eq!(output, Ok(ShapeType::Double));
    }

    #[test]
    fn float() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/float"));
        assert_eq!(output, Ok(ShapeType::Float));
    }

    #[test]
    fn long() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/long"));
        assert_eq!(output, Ok(ShapeType::Long));
    }

    #[test]
    fn timestamp() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/timestamp"));
        assert_eq!(output, Ok(ShapeType::Timestamp));
    }

    #[test]
    fn blob_stream() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/blob-stream"));
        assert_eq!(output, Ok(ShapeType::Blob(Blob(true))));
    }

    #[test]
    fn integer_bound() {
        let output = ShapeType::parse(&fixture_integer("MemorySize"));
        assert_eq!(output, Ok(ShapeType::Integer(Integer{
            min: Some(128),
            max: Some(1536)
        })));
    }

    #[test]
    fn integer_lower_bound() {
        let output = ShapeType::parse(&fixture_integer("Timeout"));
        assert_eq!(output, Ok(ShapeType::Integer(Integer{
            min: Some(1),
            max: None
        })));
    }

    #[test]
    fn integer_unbounded() {
        let output = ShapeType::parse(&fixture_integer("HttpStatus"));
        assert_eq!(output, Ok(ShapeType::Integer(Integer{
            min: None,
            max: None
        })));
    }

    #[test]
    fn list() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/list"));
        assert_eq!(output, Ok(ShapeType::List(List("AliasConfiguration".to_string()))));
    }

    #[test]
    fn string_pattern_handler() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/string-pattern-handler"));
        assert_eq!(output, Ok(ShapeType::StringPattern(StringPattern {
            pattern: "[^\\s]+".to_string(),
            min: None,
            max: Some(128),
        })));
    }

    #[test]
    fn string_pattern_description() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/string-pattern-description"));
        assert_eq!(output, Ok(ShapeType::StringPattern(StringPattern {
            pattern: ".*".to_string(),
            min: None,
            max: Some(256),
        })));
    }

    #[test]
    fn string_pattern_alias() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/string-pattern-alias"));
        assert_eq!(output, Ok(ShapeType::StringPattern(StringPattern {
            pattern: "(?!^[0-9]+$)([a-zA-Z0-9-_]+)".to_string(),
            min: Some(1),
            max: Some(128),
        })));
    }

    #[test]
    fn string_pattern_action() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/string-pattern-action"));
        assert_eq!(output, Ok(ShapeType::StringPattern(StringPattern {
            pattern: "(lambda:[*]|lambda:[a-zA-Z]+|[*])".to_string(),
            min: None,
            max: None,
        })));
    }

    #[test]
    fn string_pattern() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/string-pattern"));
        assert_eq!(output, Ok(ShapeType::StringPattern(StringPattern {
            pattern: ".*".to_string(),
            min: None,
            max: None,
        })));
    }

    #[test]
    fn string_enum() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/string-enum"));
        assert_eq!(output, Ok(ShapeType::StringEnum(StringEnum(vec!(
            "TRIM_HORIZON".to_string(),
            "LATEST".to_string(),
        )))));
    }

    #[test]
    fn invalid_type() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/invalid-type"));
        assert_eq!(output, Err(ParseError::InvalidTypeString));
    }

    // TODO -- Genericize and move to testhelpers
    fn assert_has_member(haystack: &Vec<Member>, needle: Member) {
        for member in haystack {
            if needle == *member {
                return;
            }
        }
        panic!("Member not found: {:?}", needle);
    }

    #[test]
    fn structure_add_permission_request() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/structure-add-permission-request"));
        match output.unwrap() {
            ShapeType::Structure(Structure(members)) => {
                assert_has_member(&members, Member {
                    name: "FunctionName".to_string(),
                    required: true,
                    documentation: Some("<p>Name of the Lambda function whose resource policy you are updating by adding a new permission.</p> <p> You can specify an unqualified function name (for example, \"Thumbnail\") or you can specify Amazon Resource Name (ARN) of the function (for example, \"arn:aws:lambda:us-west-2:account-id:function:ThumbNail\"). AWS Lambda also allows you to specify only the account ID qualifier (for example, \"account-id:Thumbnail\"). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 character in length. </p>".to_string()),
                    shape: "FunctionName".to_string(),
                    location: Location::URI("FunctionName".to_string()),
                });
                assert_has_member(&members, Member {
                    name: "StatementId".to_string(),
                    required: true,
                    documentation: Some("<p>A unique statement identifier.</p>".to_string()),
                    shape: "StatementId".to_string(),
                    location: Location::Body,
                });
                assert_has_member(&members, Member {
                    name: "Action".to_string(),
                    required: true,
                    documentation: Some("<p>The AWS Lambda action you want to allow in this statement. Each Lambda action is a string starting with \"lambda:\" followed by the API name (see <a>Operations</a>). For example, \"lambda:CreateFunction\". You can use wildcard (\"lambda:*\") to grant permission for all AWS Lambda actions. </p>".to_string()),
                    shape: "Action".to_string(),
                    location: Location::Body,
                });
                assert_has_member(&members, Member {
                    name: "Principal".to_string(),
                    required: true,
                    documentation: Some("<p>The principal who is getting this permission. It can be Amazon S3 service Principal (\"s3.amazonaws.com\") if you want Amazon S3 to invoke the function, an AWS account ID if you are granting cross-account permission, or any valid AWS service principal such as \"sns.amazonaws.com\". For example, you might want to allow a custom application in another AWS account to push events to AWS Lambda by invoking your function. </p>".to_string()),
                    shape: "Principal".to_string(),
                    location: Location::Body,
                });
                assert_has_member(&members, Member {
                    name: "SourceArn".to_string(),
                    required: false,
                    documentation: Some("<p>This is optional; however, when granting Amazon S3 permission to invoke your function, you should specify this field with the bucket Amazon Resource Name (ARN) as its value. This ensures that only events generated from the specified bucket can invoke the function. </p> <important>If you add a permission for the Amazon S3 principal without providing the source ARN, any AWS account that creates a mapping to your function ARN can send events to invoke your Lambda function from Amazon S3.</important>".to_string()),
                    shape: "Arn".to_string(),
                    location: Location::Body,
                });
                assert_has_member(&members, Member {
                    name: "SourceAccount".to_string(),
                    required: false,
                    documentation: Some("<p>The AWS account ID (without a hyphen) of the source owner. For example, if the <code>SourceArn</code> identifies a bucket, then this is the bucket owner's account ID. You can use this additional condition to ensure the bucket you specify is owned by a specific account (it is possible the bucket owner deleted the bucket and some other AWS account created the bucket). You can also use this condition to specify all sources (that is, you don't specify the <code>SourceArn</code>) owned by a specific account. </p>".to_string()),
                    shape: "SourceOwner".to_string(),
                    location: Location::Body,
                });
                assert_has_member(&members, Member {
                    name: "Qualifier".to_string(),
                    required: false,
                    documentation: Some("<p>You can specify this optional query parameter to specify function version or alias name. The permission will then apply to the specific qualified ARN. For example, if you specify function version 2 as the qualifier, then permission applies only when request is made using qualified function ARN: </p> <p><code>arn:aws:lambda:aws-region:acct-id:function:function-name:2</code></p> <p>If you specify alias name, for example \"PROD\", then the permission is valid only for requests made using the alias ARN:</p> <p><code>arn:aws:lambda:aws-region:acct-id:function:function-name:PROD</code></p> <p>If the qualifier is not specified, the permission is valid only when requests is made using unqualified function ARN. </p> <p><code>arn:aws:lambda:aws-region:acct-id:function:function-name</code></p>".to_string()),
                    shape: "Qualifier".to_string(),
                    location: Location::QueryString("Qualifier".to_string()),
                });
            }
            _ => panic!("Wrong type")
        }
    }

    #[test]
    fn exception_too_many_requests() {
        let output = ShapeType::parse(&fixture_btreemap("shape-types/exception-too-many-requests"));
        match output.unwrap() {
            ShapeType::Exception(e) => {
                assert_eq!(e.documentation, None);
                assert_eq!(e.members.len(), 3);
                assert_eq!(e.status_code, 429);
                assert_has_member(&e.members, Member {
                    name: "retryAfterSeconds".to_string(),
                    required: false,
                    shape: "String".to_string(),
                    documentation: Some("<p>The number of seconds the caller should wait before retrying.</p>".to_string()),
                    location: Location::Header("Retry-After".to_string()),
                });
                assert_has_member(&e.members, Member {
                    name: "Type".to_string(),
                    required: false,
                    shape: "String".to_string(),
                    documentation: None,
                    location: Location::Body,
                });
                assert_has_member(&e.members, Member {
                    name: "message".to_string(),
                    required: false,
                    shape: "String".to_string(),
                    documentation: None,
                    location: Location::Body,
                });
            }
            _ => panic!("Wrong type!")
        }
    }
}
