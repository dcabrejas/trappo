use self::error::StepError;
use super::cmd::*;
use super::Operation;
use config::stages::HostConfig;
use std::io::{Error, ErrorKind};

pub mod core;
pub mod error;
pub mod git;
pub mod rollback;

#[derive(Debug)]
pub struct Context {
    pub config: HostConfig,
    pub releases_path: String,
    pub release_path: String,
    pub prev_release_path: Option<String>,
    pub shared_path: String,
}

impl Context {
    pub fn from_host_config(config: HostConfig, opt: Operation) -> Result<Context, Error> {
        match opt {
            Operation::Deploy => Context::deploy_context(config),
            Operation::Rollback => Context::rollback_context(config),
        }
    }

    fn rollback_context(config: HostConfig) -> Result<Context, Error> {
        let shared_path = format!("{}/shared", config.deploy_path);
        let releases_path = format!("{}/releases", config.deploy_path);
        let existing_releases = exec_remote_fetch_sorted_filenames_in_dir(
            &config.host,
            &releases_path,
            SortOrder::Des,
        )?;

        //get the current release currently on the server.
        let release_path = existing_releases
            .get(0)
            .map(|name| format!("{}/{}", releases_path, name));

        let release_path = if let Some(path) = release_path {
            path
        } else {
            return Err(Error::new(
                ErrorKind::Other,
                "There are no releases on the server",
            ));
        };

        //get the previous release (target for rollback).
        let prev_release_path = existing_releases
            .get(1)
            .map(|name| format!("{}/{}", releases_path, name));

        Ok(Context {
            config,
            releases_path,
            release_path,
            prev_release_path,
            shared_path,
        })
    }

    fn deploy_context(config: HostConfig) -> Result<Context, Error> {
        let output = exec_remote_cmd(&config.host, "date +'%Y%m%d%H%M%S'")?;

        if !output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                "Failed to compute current timestamp at the server",
            ));
        }

        let releases_path = format!("{}/releases", config.deploy_path);
        let existing_releases = exec_remote_fetch_sorted_filenames_in_dir(
            &config.host,
            &releases_path,
            SortOrder::Des,
        )?;

        //get the newest release currently on the server (used in case of rollback).
        let prev_release_path = existing_releases
            .get(0)
            .map(|name| format!("{}/{}", releases_path, name));

        let release_timestamp = String::from_utf8_lossy(&output.stdout);
        let releases_path = format!("{}/releases", config.deploy_path);
        let release_path = format!("{}/{}", releases_path, release_timestamp);
        let shared_path = format!("{}/shared", config.deploy_path);

        Ok(Context {
            config,
            releases_path,
            release_path,
            prev_release_path,
            shared_path,
        })
    }
}

pub trait Step {
    fn execute(&self, context: &Context) -> Result<(), StepError>;
    fn get_name(&self) -> &str;
}
