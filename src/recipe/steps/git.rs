use super::{Step, Context};
use super::error::*;
use super::super::super::cmd::*;

pub struct GitClone { name: &'static str }

impl Step for GitClone {

    fn new(name: &'static str) -> GitClone {
        Self { name }
    }

    fn execute (&self, context: &Context) -> Result<(), StepError> {
        let server_command = format!(
            "git clone {} {}",
            context.config.repo_url.trim(),
            context.release_path.trim(),
        );

        exec_remote_cmd_inherit_output(&context.config.host, &server_command)?;

        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
