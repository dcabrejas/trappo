//! Contains the default steps performed during a rollback.
use steps::{Step, Context, error::StepError};
use cmd::*;

const NO_PREV_ERR_MSG: &str = "Can't rollback because there is no release to rollback to.";

/// Checks whether or not there is a release to rollback to.
pub struct CanRollBack;

/// Checks whether or not there is a release to rollback to. If the `Context.prev_release_path`
/// contains a `None`, then the rollback is not possible.
impl Step for CanRollBack {

    fn execute (&self, context: &Context) -> Result<(), StepError> {
        context.prev_release_path.as_ref().ok_or(StepError::Critical(NO_PREV_ERR_MSG.into())).map(|_| ())
    }

    fn get_name(&self) -> &str {
        "rollback:check"
    }
}

/// Removes all the files from the relase we are rolling back from.
/// Note this doesn't include the shared files and directories which are shared
/// accross releases.
pub struct RemoveCurrentRelease;

/// Removes all the files from the relase we are rolling back from.
impl Step for RemoveCurrentRelease {

    fn execute (&self, context: &Context) -> Result<(), StepError> {

        let release_path = &context.release_path;
        let cmd = format!("rm -rf {}", release_path);
        let output = exec_remote_cmd(&context.config.host, &cmd)?;

        match output.status.success() {
            true => Ok(()),
            false => {
                let err_msg = format!("Failed to remove current release at {}", release_path);
                Err(StepError::Critical(err_msg))
            }
        }
    }

    fn get_name(&self) -> &str {
        "rollback:rm:latest"
    }
}

/// Points the current symlink to the previous relases, completing the rollback.
pub struct SymlinkPreviousRelease;

impl Step for SymlinkPreviousRelease {

    fn execute (&self, context: &Context) -> Result<(), StepError> {

        let current_symlink_path = format!("{}/current", context.config.deploy_path);
        let current_symlink_exist = exec_remote_file_exists(&context.config.host, &current_symlink_path, FSResourceType::Symlink)?;

        //remove current symlink if it exists.
        if current_symlink_exist {
            let remove_current_command = format!("rm {}", current_symlink_path);

            let status = exec_remote_cmd(&context.config.host, &remove_current_command)?.status;

            if !status.success() {
                return Err(StepError::fromFailedCommand(&remove_current_command, status.code()));
            }
        }

        let prev_release_path = context.prev_release_path.as_ref().unwrap();
        let create_current_symlink_cmd = format!("ln -s {} {}", prev_release_path.trim(), current_symlink_path);
        let status = exec_remote_cmd(&context.config.host, &create_current_symlink_cmd)?.status;

        if !status.success() {
            return Err(StepError::fromFailedCommand(&create_current_symlink_cmd, status.code()));
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        "rollback:symlink:previous"
    }
}
