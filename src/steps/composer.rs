use steps::{Step, Context, error::StepError};
use cmd::*;

pub struct ComposerInstall;

impl Step for ComposerInstall {

    fn execute (&self, context: &Context) -> Result<(), StepError> {
        let server_command = format!("cd {} && composer install", context.release_path.trim());

        let status = exec_remote_cmd_inherit_output(&context.config.host, &server_command)?;

        if !status.success() {
            return Err(StepError::fromFailedCommand(&server_command, status.code()));
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        "composer:install"
    }
}
