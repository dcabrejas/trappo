use cmd::*;
use steps::{error::StepError, Context, Step};

pub struct GitClone;

impl Step for GitClone {
    fn execute(&self, context: &Context) -> Result<(), StepError> {
        let server_command = format!(
            "git clone {} {}",
            context.config.repo_url.trim(),
            context.release_path.trim(),
        );

        let status = exec_remote_cmd_inherit_output(&context.config.host, &server_command)?;

        if !status.success() {
            return Err(StepError::from_failed_command(
                &server_command,
                status.code(),
            ));
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        "git:clone"
    }
}
