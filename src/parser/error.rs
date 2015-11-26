#[derive(Debug, PartialEq)]
pub enum ParseError {
    ExpectedObject,
    TypeStringMissing,
    NotImplemented,
    InvalidTypeString,
    StructureHasNoMembers,
    InvalidStructureMembers,
    InvalidMember,
    InvalidRequired,
    MissingErrorInException,
    InvalidMaxInteger,
    InvalidMinInteger,
}
