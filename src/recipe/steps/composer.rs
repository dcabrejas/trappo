use super::{Step, Context};
use super::error::*;
use super::super::super::cmd::*;

pub struct ComposerInstall { name: &'static str }

impl Step for ComposerInstall {

    fn new(name: &'static str) -> ComposerInstall {
        Self { name }
    }

    fn execute (&self, context: &Context) -> Result<(), StepError> {
        let server_command = format!(
            "cd {} && composer install",
            context.release_path.trim()
        );

        exec_remote_cmd_inherit_output(&context.config.host, &server_command)?;

        Ok(())
    }

    fn get_name(&self) -> &str {
        self.name
    }
}
