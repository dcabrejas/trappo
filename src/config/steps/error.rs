use std::fmt;

use self::StepConfigError::*;

#[derive(Debug)]
pub enum StepConfigError {
    ///Parameters: (entry_name, expected_type)
    BadType(String, String),
    ///Parameters: (field_name)
    MissingField(String),
    //The position of the step is ambiguous
    AmbiguousPosition,
    //The position field is missing from the configuration.
    MissingPosition,
    IoError,
}

impl fmt::Display for StepConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BadType(ref name, ref expected) => {
                write!(f, "type mismatch for '{}'. expected {}", name, expected)
            },
            MissingField(ref name) => write!(f, "Required field '{}' is missing from steps config file", name),
            AmbiguousPosition => write!(f, "Ambiguous step position, only one of before and after fields is allowed, not both."),
            MissingPosition => write!(f, "Step postion is missing, define the position of the step using \"before\" or \"after\" fields."),
            IoError => write!(f, "I/O error while reading the config file"),
        }
    }
}
