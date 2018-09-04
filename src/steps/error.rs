use std::io;
use std::error::Error;

#[derive(Debug)]
pub enum StepError {
    ///Critical error
    Critical(String),
    ///Non-critical error
    NonCritical(String)
}

impl StepError {
    pub fn fromFailedCommand(cmd: &str, status: Option<i32>) -> Self {
        let error_msg = match status {
            Some(code) => format!("Command '{}' exited with non-sucessful status code '{}'", cmd, code),
            None => format!("Command '{}' exited with non-sucessful status code", cmd)
        };

        StepError::Critical(error_msg)
    }

    pub fn nonCriticalfromError<E: Error>(error: E) -> Self {
        StepError::NonCritical(error.to_string())
    }
}

impl From<io::Error> for StepError {
    fn from(error: io::Error) -> Self {
        StepError::Critical(error.to_string())
    }
}
