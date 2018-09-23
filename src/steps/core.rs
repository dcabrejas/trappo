//! Contains the core steps included out of the box which are used during a deployment
//! when the recipe builder method `.with_core_steps` is invoked.
//!
//! When a deployment is run, there are certain steps that are necessary for every deployment.
//! These include things like initial set up of directories, setting up symlinks and cleaning up old releases
//! after a successfull deploy.
//!
//! The only exception is the `RawCmdStep`. This struct is what commands feed to the program using an external
//! `.deploy_rs/steps.toml` file end up being. And they can be executed during deployment by invoking the
//! recipe builder's `.with_steps_from_file(file_name: String)` method. <- TODO implementation.

use steps::{Step, Context, error::StepError};
use config::steps::StepConfig;
use cmd::*;
use display::*;

/// This struct is what commands feed to the program using an external
/// `.deploy_rs/steps.toml` file end up being. And they can be executed during deployment by invoking the
/// recipe builder's `.with_steps_from_file(file_name: String)` method.
///
/// It executes the command provided in the config file directly on the server, if the command fails
/// it returns a critial error which will trigger a rollback. The output of the command is shown in the terminal.
pub struct RawCmdStep { name: String, raw_cmd: String }

impl Step for RawCmdStep {

    fn execute (&self, context: &Context) -> Result<(), StepError> {

        let server_command = format!("cd {} && {}", context.release_path.trim(), self.raw_cmd);
        let status = exec_remote_cmd_inherit_output(&context.config.host, &server_command)?;

        if !status.success() {
            return Err(StepError::fromFailedCommand(&self.raw_cmd, status.code()));
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}

impl From<StepConfig> for RawCmdStep {
    fn from(config_step: StepConfig) -> Self {
        RawCmdStep { name: config_step.name, raw_cmd: config_step.comand}
    }
}

/// It performs the initial setup on the server prior to deployment.
/// Basically creating the deployment path folder if it doesn't already exist.
pub struct InitStep;

impl Step for InitStep {

    fn execute (&self, context: &Context) -> Result<(), StepError> {
        let create_release_path_cmd = format!("mkdir -p {}", context.release_path);

        let status = exec_remote_cmd(&context.config.host, &create_release_path_cmd)?.status;

        if !status.success() {
            return Err(StepError::fromFailedCommand(&create_release_path_cmd, status.code()));
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        "core:init"
    }
}

pub struct LinkFiles;

impl Step for LinkFiles {

    fn execute (&self, context: &Context) -> Result<(), StepError> {
        
        for file in context.config.link_files.iter() {
            let shared_file_path = format!("{}/{}", context.shared_path, file);
            let symlink_path     = format!("{}/{}", context.release_path.trim(), file);

            let file_exists = exec_remote_file_exists(&context.config.host, &shared_file_path, FSResourceType::File)?;

            if !file_exists {
                let error_msg = format!("Could not create symlink for file {} because it doesn't exist", file);
                return Err(StepError::Critical(error_msg));
            }

            let symlink_command = format!("ln -s {} {}",shared_file_path, symlink_path);

            let status = exec_remote_cmd(&context.config.host, &symlink_command)?.status;

            if !status.success() {
                return Err(StepError::fromFailedCommand(&symlink_command, status.code()));
            }
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        "core:link:files"
    }
}

pub struct LinkDirs;

impl Step for LinkDirs {

    fn execute (&self, context: &Context) -> Result<(), StepError> {

        for dir in context.config.link_dirs.iter() {
            let shared_dir_path = format!("{}/{}", context.shared_path, dir);
            let symlink_path    = format!("{}/{}", context.release_path.trim(), dir);

            let dir_exists = exec_remote_file_exists(&context.config.host, &shared_dir_path, FSResourceType::Directory)?;

            if !dir_exists {
                let error_msg = format!("Could not create symlink for dir {} because it doesn't exist", dir);
                return Err(StepError::Critical(error_msg));
            }

            let symlink_command = format!("ln -s {} {}", shared_dir_path, symlink_path);
            let status = exec_remote_cmd(&context.config.host, &symlink_command)?.status;

            if !status.success() {
                return Err(StepError::fromFailedCommand(&symlink_command, status.code()));
            }
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        "core:link:directories"
    }
}

pub struct SymlinkCurrent;

impl Step for SymlinkCurrent {

    fn execute (&self, context: &Context) -> Result<(), StepError> {

        let current_symlink_path = format!("{}/current", context.config.deploy_path);

        let current_symlink_exist = exec_remote_file_exists(&context.config.host, &current_symlink_path, FSResourceType::Symlink)?;

        if current_symlink_exist {
            let remove_current_command = format!("rm {}", current_symlink_path);

            let status = exec_remote_cmd(&context.config.host, &remove_current_command)?.status;

            if !status.success() {
                return Err(StepError::fromFailedCommand(&remove_current_command, status.code()));
            }
        }

        let create_current_symlink_cmd = format!("ln -s {} {}", context.release_path.trim(), current_symlink_path);

        let status = exec_remote_cmd(&context.config.host, &create_current_symlink_cmd)?.status;

        if !status.success() {
            return Err(StepError::fromFailedCommand(&create_current_symlink_cmd, status.code()));
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        "core:link:current"
    }
}

pub struct CleanUpReleases;

impl Step for CleanUpReleases {

    fn execute (&self, context: &Context) -> Result<(), StepError> {

        let mut releases   = exec_remote_fetch_sorted_filenames_in_dir(&context.config.host, &context.releases_path, SortOrder::Asc)
            .map_err(|e| StepError::nonCriticalfromError(e))?;

        let keep_releases  = context.config.keep_releases as usize;
        let total_releases = releases.len();

        if total_releases <= keep_releases { return Ok(()); };
        let to_remove = total_releases - keep_releases;
        releases.resize(to_remove, "".into());

        for release_dir in &releases {
            let delete_dir_cmd = format!("rm -rf {}/{}", &context.releases_path, release_dir);
            let output = exec_remote_cmd(&context.config.host, &delete_dir_cmd)?;

            if !output.status.success() {
                render_error(&format!("Failed to clean up old release {}", release_dir));
            }

            println!("Deleted: {}", release_dir);
        }

        Ok(())
    }


    fn get_name(&self) -> &str {
        "core:cleanup:releases"
    }
}
