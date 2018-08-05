use std::process::{Command, Stdio, Output, ExitStatus};
use std::io;

pub enum FSResourceType {
    File,
    Directory,
    Symlink
}

pub fn exec_remote_cmd(host: &str, cmd: &str) -> Result<Output, io::Error> {

    let output = Command::new("ssh").args(&[host, cmd]).output()?;

    Ok(output)
}

pub fn exec_remote_cmd_inherit_output(host: &str, cmd: &str) -> Result<ExitStatus, io::Error> {

    let mut child = Command::new("ssh")
        .args(&[host, cmd])
        .stdout(Stdio::inherit())
        .spawn()?;

    let status = child.wait()?;

    Ok(status)
}

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

pub fn exec_remote_fetch_sorted_filenames_in_dir(host: &str, dir_path: &str) -> Result<Vec<String>, io::Error> {

    let cmd = format!("find {} -maxdepth 1 -mindepth 1 -printf '%f\n' | sort", dir_path);
    let output = exec_remote_cmd(host, &cmd)?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    let vector: Vec<String> = stdout.trim_right().split("\n").map(|x| String::from(x)).collect();

    Ok(vector)
}
