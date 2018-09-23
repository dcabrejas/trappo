//! Contains an API for running commands on a given host.
//!
//! These functions are mostly used by deployment steps to perform
//! the different actions they need to do while deployment or rolling back.

use std::process::{Command, Stdio, Output, ExitStatus};
use std::io;

pub enum FSResourceType {
    File,
    Directory,
    Symlink
}

pub enum SortOrder {
    Asc,
    Des,
}

/// Executes a command on the server and returns its output or an IO error.
pub fn exec_remote_cmd(host: &str, cmd: &str) -> Result<Output, io::Error> {

    let output = Command::new("ssh").args(&[host, cmd]).output()?;

    Ok(output)
}

/// Executes a command on the server and pipes its output directly to stdout.
/// It returns the command `ExitStatus` or an IO error.
pub fn exec_remote_cmd_inherit_output(host: &str, cmd: &str) -> Result<ExitStatus, io::Error> {

    let mut child = Command::new("ssh")
        .args(&[host, cmd])
        .stdout(Stdio::inherit())
        .spawn()?;

    let status = child.wait()?;

    Ok(status)
}

/// Checks whether or not a given file exist on the server.
/// Allows checking for different types including File, Directory and Symlink.
pub fn exec_remote_file_exists(host: &str, file_path: &str, resource_type: FSResourceType) -> Result<bool, io::Error> {

    let type_key = match resource_type {
        FSResourceType::File => "f",
        FSResourceType::Directory => "d",
        FSResourceType::Symlink => "L"
    };

    let cmd = format!("test -{} {}", type_key, file_path);
    let output = exec_remote_cmd(host, &cmd)?;

    Ok(output.status.success())
}

/// Given a path, it returns a vector containing the filenames of all the files inside that path
/// sorted in descending order.
pub fn exec_remote_fetch_sorted_filenames_in_dir(host: &str, dir_path: &str, sort_order: SortOrder) -> Result<Vec<String>, io::Error> {

    let sort_cmd = match sort_order {
        SortOrder::Asc => "sort",
        SortOrder::Des => "sort -r"
    };

    let cmd = format!("find {} -maxdepth 1 -mindepth 1 -printf '%f\n' | {}", dir_path, sort_cmd);
    let output = exec_remote_cmd(host, &cmd)?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    let vector: Vec<String> = stdout.trim_right().split("\n").map(|x| String::from(x)).collect();

    Ok(vector)
}
