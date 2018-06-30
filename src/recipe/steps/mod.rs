use config::HostConfig;
use std::process::Command;
use std::io::{Error, ErrorKind};

pub mod core;
pub mod git;
pub mod composer;
pub mod error;

#[derive(Debug)]
pub struct Context {
    pub config: HostConfig,
    pub release_path: String,
    pub shared_path: String,
}

impl Context {
    pub fn from_host_config(config: HostConfig) -> Result<Context, Error> {

        let output = Command::new("ssh")
            .args(&[&config.host, "date +'%Y%m%d%H%M%S'"])
            .output()?;

        if !output.status.success() {
            Err(Error::new(ErrorKind::Other, "Failed to compute current timestamp at the server"))
        } else {
            let release_timestamp = String::from_utf8_lossy(&output.stdout);
            let release_path      = format!("{}/releases/{}", config.deploy_path, release_timestamp);
            let shared_path       = format!("{}/shared", config.deploy_path);

            Ok(Context { config, release_path, shared_path })
        }
    }
}

pub trait Step {
    fn new(name: &'static str) -> Self where Self: Sized;
    fn execute(&self, context: &Context) -> Result<(), error::StepError>;
    fn get_name(&self) -> &str;
}

//return the steps you want to execute
pub fn get_steps() -> Vec<Box<Step>>{

    let steps: Vec<Box<Step>> = vec![
        Box::new(self::core::SetUpStep::new("core:setup")),
        Box::new(self::git::GitClone::new("git:clone")),
        Box::new(self::composer::ComposerInstall::new("composer:install")),
        Box::new(self::core::LinkFiles::new("core:link:files")),
        Box::new(self::core::LinkDirs::new("core:link:directories")),
    ];

    steps
}
