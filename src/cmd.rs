use std::process::{Command, Stdio, Output};
use super::recipe::steps::error::StepError;
use std::error::Error;

pub enum FSResourceType {
    File,
    Directory
}

pub fn exec_remote_cmd(host: &str, cmd: &str) -> Result<Output, StepError> {

    let output = Command::new("ssh")
        .args(&[host, cmd])
        .output()
        .map_err(|error| {
            StepError::IO(error.description().into())
        })?;

    if !output.status.success() {
        return Err(StepError::StatusCode(cmd.into(), output.status.code().unwrap_or(0)));
    }

    Ok(output)
}

pub fn exec_remote_cmd_inherit_output(host: &str, cmd: &str) -> Result<(), StepError> {

    let mut child = Command::new("ssh")
        .args(&[host, cmd])
        .stdout(Stdio::inherit())
        .spawn()
        .map_err(|error| {
            StepError::IO(error.description().into())
        })?;

    let status = child.wait().unwrap();

    if !status.success() {
        return Err(StepError::StatusCode(cmd.into(), status.code().unwrap_or(0)));
    }

    Ok(())
}

pub fn exec_remote_file_exists(host: &str, file_path: &str, resource_type: FSResourceType) -> Result<bool, StepError> {

    let typeKey = match resource_type {
        FSResourceType::File => "f",
        FSResourceType::Directory => "d"
    };

    let cmd = format!("test -{} {}", typeKey, file_path);
    exec_remote_cmd(host, &cmd)?;

    Ok(true)
}
