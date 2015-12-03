#[derive(Debug)]
/// The AWS Lambda service encountered an internal error.
pub struct ServiceException {
    pub Type: String,
    pub Message: String,
}

impl ::std::error::Error for ServiceException {
    pub fn description(&self) -> &str {
        &format!("ServiceException: {} {}", self.Type, self.Message);
    }

    pub fn cause(&self) -> Option<&Error> {
        None
    }
}
