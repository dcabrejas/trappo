use super::{Step, Context};
use super::super::super::cmd::*;

pub struct SetUpStep { name: &'static str }

impl Step for SetUpStep {
    fn new(name: &'static str) -> SetUpStep {
        Self { name }
    }

    fn execute (&self, context: &Context) -> Result<(), String> {
        let create_release_path_cmd = format!("mkdir -p {}", context.release_path);

        let output = exec_remote_cmd(&context.config.host, &create_release_path_cmd)
            .map_err(|_io_error| format!("Could not connect to the server") )?;

        if !output.status.success() {
            return Err(format!(
                "Invalid status code {} returned by command '{}' at '{}'.",
                output.status.code().unwrap_or(0),
                create_release_path_cmd,
                context.config.host
            ));
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

pub struct LinkFiles { name: &'static str }

impl Step for LinkFiles {

    fn new(name: &'static str) -> LinkFiles {
        Self { name }
    }

    fn execute (&self, context: &Context) -> Result<(), String> {

        for file in context.config.link_files.iter() {
            let shared_file_path = format!("{}/{}", context.shared_path, file);
            let symlink_path     = format!("{}/{}", context.release_path.trim(), file);

            let file_exists = exec_remote_file_exists(&context.config.host, &shared_file_path, FSResourceType::File)
                .map_err(|_io_error| format!("Could not connect to the server") )?;

            if !file_exists { return Err(format!("Could not create symlink for file {} because it doesn't exist", file)) }

            let symlink_command = format!("ln -s {} {}",shared_file_path, symlink_path);

            let output = exec_remote_cmd(&context.config.host, &symlink_command)
                .map_err(|_io_error| format!("Could not connect to the server") )?;

            if !output.status.success() {
                return Err(format!("Command '{}' exited with non-sucessful status code", symlink_command));
            }
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

pub struct LinkDirs { name: &'static str }

impl Step for LinkDirs {

    fn new(name: &'static str) -> LinkDirs {
        Self { name }
    }

    fn execute (&self, context: &Context) -> Result<(), String> {

        for dir in context.config.link_dirs.iter() {
            let shared_dir_path = format!("{}/{}", context.shared_path, dir);
            let symlink_path    = format!("{}/{}", context.release_path.trim(), dir);

            let dir_exists = exec_remote_file_exists(&context.config.host, &shared_dir_path, FSResourceType::Directory)
                .map_err(|_io_error| format!("Could not connect to the server") )?;

            if !dir_exists { return Err(format!("Could not create symlink for dir {} because it doesn't exist", dir)) }

            let symlink_command = format!("ln -s {} {}", shared_dir_path, symlink_path);

            let output = exec_remote_cmd(&context.config.host, &symlink_command)
                .map_err(|_io_error| format!("Could not connect to the server") )?;

            if !output.status.success() {
                return Err(format!("Command '{}' exited with non-sucessful status code", symlink_command));
            }
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
