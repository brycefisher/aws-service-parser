#[derive(Debug)]
/// The AWS Lambda service encountered an internal error.
pub struct ServiceException {
    Type: String,
    Message: String,
}

impl ::std::error::Error for ServiceException {
    pub fn description(&self) -> &str {
        &format!("{}: {}", self.Type, self.Message)
    }

    pub fn cause(&self) -> Option<&Error> {
        None
    }
}
