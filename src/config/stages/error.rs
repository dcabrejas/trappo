use std::fmt;

use self::ConfigError::*;

#[derive(Debug)]
pub enum ConfigError {
    ///Parameters: (config_file_name)
    NotFound(String),
    ///Parementers: (stage_name)
    BadStage(String),
    ///Parameters: (entry_name, expected_type)
    BadType(String, String),
    ///Parameters: (field_name)
    MissingField(String),
    IoError
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotFound(ref file) => write!(f, "Config file '{}' was not found", file),
            BadStage(ref env) => write!(f, "Couldn't find stage '{}' in config file", env),
            BadType(ref name, ref expected) => {
                write!(f, "type mismatch for '{}'. expected {}", name, expected)
            },
            MissingField(ref name) => write!(f, "Required field '{}' is missing from the config file", name),
            IoError => write!(f, "I/O error while reading the config file"),
        }
    }
}
