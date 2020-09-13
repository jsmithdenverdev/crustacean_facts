use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct Error(pub String);

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
}
