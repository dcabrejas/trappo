use super::{Step, Context};
use super::error::*;
use super::super::super::cmd::*;

pub struct SetUpStep { name: &'static str }

impl Step for SetUpStep {
    fn new(name: &'static str) -> SetUpStep {
        Self { name }
    }

    fn execute (&self, context: &Context) -> Result<(), StepError> {
        let server_command = format!(
            "mkdir -p {}",
            context.release_path
        );

        let _output = exec_remote_cmd(&context.config.host,&server_command)?;

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

    fn execute (&self, context: &Context) -> Result<(), StepError> {

        for file in context.config.link_files.iter() {
            let shared_file_path = format!("{}/{}", context.shared_path, file);
            let symlink_path     = format!("{}/{}", context.release_path.trim(), file);

            //check if shared file exists, otherwise return error
            exec_remote_file_exists(&context.config.host, &shared_file_path, FSResourceType::File)?;

            let symlink_command = format!("ln -s {} {}",shared_file_path, symlink_path);

            exec_remote_cmd(&context.config.host, &symlink_command)?;
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

    fn execute (&self, context: &Context) -> Result<(), StepError> {

        for dir in context.config.link_dirs.iter() {
            let shared_dir_path = format!("{}/{}", context.shared_path, dir);
            let symlink_path    = format!("{}/{}", context.release_path.trim(), dir);

            //check if shared dir exists, otherwise return error
            exec_remote_file_exists(&context.config.host, &shared_dir_path, FSResourceType::Directory)?;

            let symlink_command = format!("ln -s {} {}",shared_dir_path, symlink_path);

            exec_remote_cmd(&context.config.host, &symlink_command)?;
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
