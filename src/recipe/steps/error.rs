use std::error::Error;
use std::convert::From;
use std;

#[derive(Debug, Clone)]
pub enum StepError {
    StatusCode(String, i32),
    IO(String),
}

/**
impl Error for StepError {
    fn description(&self) -> &str {
        match self {
            StepError::StatusCode(process_name, code) => format!("Command '{}' failed with status code {}", process_name, code).as_str(),
            StepError::IO(message) => &format!("IO error : {}", message).as_str()
        }
    }
}

impl std::fmt::Display for StepError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StepError::StatusCode(process_name, code) => write!(f, "Command '{}' failed with status code {}", process_name, code),
            StepError::IO(message) => write!(f, "IO error : {}", message)
        }

    }
}
**/

impl From<std::io::Error> for StepError {
    fn from(error: std::io::Error) -> Self {
        StepError::IO(error.description().into())
    }
}