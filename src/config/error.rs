use std::error::Error;
use std::convert::From;
use std;

#[derive(Debug)]
pub struct ParseError {
    description: String
}

impl ParseError {
    pub fn new(description: String) -> ParseError {
        ParseError {description}
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error Description: {}", &self.description)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(error: std::io::Error) -> Self {
        ParseError::new(error.description().into())
    }
}