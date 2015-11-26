#[derive(Debug, PartialEq)]
pub enum ParseError {
    ExpectedObject,
    TypeStringMissing,
    NotImplemented,
    InvalidTypeString,
    StructureHasNoMembers,
    InvalidStructureMembers,
    MissingListShape,
    InvalidListShape,
    MissingListMember,
    InvalidListMember,
    InvalidMember,
    InvalidRequired,
    MissingErrorInException,
    InvalidMaxInteger,
    InvalidMinInteger,
}
