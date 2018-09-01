use steps::{Step, Context};
use cmd::*;

pub struct GitClone;

impl Step for GitClone {

    fn execute (&self, context: &Context) -> Result<(), String> {
        let server_command = format!(
            "git clone {} {}",
            context.config.repo_url.trim(),
            context.release_path.trim(),
        );

        let status = exec_remote_cmd_inherit_output(&context.config.host, &server_command)
            .map_err(|_io_error| format!("Could not connect to the server") )?;

        if !status.success() {
            return Err(format!(
                "Invalid status code {} returned by command '{}' at '{}'.",
                status.code().unwrap_or(0),
                server_command,
                context.config.host
            ));
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        "git:clone"
    }
}