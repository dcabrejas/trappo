use steps::{Step, Context};
use config::steps::StepConfig;
use cmd::*;
use display::*;

pub struct RawCmdStep { name: String, raw_cmd: String }

impl Step for RawCmdStep {

    fn execute (&self, context: &Context) -> Result<(), String> {

        let status = exec_remote_cmd_inherit_output(&context.config.host, &self.raw_cmd)
            .map_err(|_io_error| format!("Could not connect to the server") )?;

        if !status.success() {
            return Err(format!(
                "Invalid status code {} returned by command '{}' at '{}'.",
                status.code().unwrap_or(0),
                self.raw_cmd,
                context.config.host
            ));
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

pub struct InitStep;

impl Step for InitStep {

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
        "core:init"
    }
}

pub struct LinkFiles;

impl Step for LinkFiles {

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
        "core:link:files"
    }
}

pub struct LinkDirs;

impl Step for LinkDirs {

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
        "core:link:directories"
    }
}

pub struct SymlinkCurrent;

impl Step for SymlinkCurrent {

    fn execute (&self, context: &Context) -> Result<(), String> {

        let current_symlink_path = format!("{}/current", context.config.deploy_path);

        let current_symlink_exist = exec_remote_file_exists(&context.config.host, &current_symlink_path, FSResourceType::Symlink)
            .map_err(|_io_error| format!("Could not connect to the server") )?;

        if current_symlink_exist {
            let remove_current_command = format!("rm {}", current_symlink_path);

            let output = exec_remote_cmd(&context.config.host, &remove_current_command)
                .map_err(|_io_error| format!("Could not connect to the server") )?;

            if !output.status.success() {
                return Err(format!("Command '{}' exited with non-sucessful status code", remove_current_command));
            }
        }

        let create_current_symlink_cmd = format!("ln -s {} {}", context.release_path.trim(), current_symlink_path);

        let output = exec_remote_cmd(&context.config.host, &create_current_symlink_cmd)
            .map_err(|_io_error| format!("Could not connect to the server") )?;

        if !output.status.success() {
            return Err(format!("Command '{}' exited with non-sucessful status code", create_current_symlink_cmd));
        }

        Ok(())
    }

    fn get_name(&self) -> &str {
        "core:link:current"
    }
}

pub struct CleanUpReleases;

impl Step for CleanUpReleases {

    fn execute (&self, context: &Context) -> Result<(), String> {

        let mut releases = exec_remote_fetch_sorted_filenames_in_dir(&context.config.host, &context.releases_path)
            .map_err(|_io_error| format!("Could not connect to the server") )?;

        let keep_releases  = context.config.keep_releases as usize;
        let total_releases = releases.len();

        if total_releases <= keep_releases {
            return Ok(());
        };

        let to_remove = total_releases - keep_releases;

        println!("total releases : {}", total_releases);
        println!("to remove : {}", to_remove);

        releases.resize(to_remove, "".into());

        for release_dir in &releases {
            let delete_dir_cmd = format!("rm -rf {}/{}", &context.releases_path, release_dir);
            let output = exec_remote_cmd(&context.config.host, &delete_dir_cmd)
                .map_err(|_io_error| format!("Could not connect to the server") )?;

            if !output.status.success() {
                render_error(&format!("Failed to clean up old release {}", release_dir));
            }

            println!("deleted {}", release_dir);
        }

        Ok(())
    }


    fn get_name(&self) -> &str {
        "core:cleanup:releases"
    }
}
