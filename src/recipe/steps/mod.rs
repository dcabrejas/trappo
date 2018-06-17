use super::super::config::HostConfig;
use std::process::Command;

pub mod core;

pub struct Context {
    pub config: HostConfig,
    pub release_path: String
}

impl Context {
    pub fn from_host_config(config: HostConfig) -> Context {

        let output = Command::new("ssh")
            .args(&[&config.host, "date +'%Y%m%d%H%M%S'"])
            .output()
            .expect("Failed to get current timestamp from the server");

        let release_timestamp = String::from_utf8_lossy(&output.stdout);
        let release_path = format!("{}/releases/{}", config.deploy_path, release_timestamp);

        Context { config, release_path }
    }
}

pub trait Step {
    fn new(name: &'static str) -> Self where Self: Sized;
    fn execute(&self, context: &Context) -> Result<(), &str>;
    fn get_name(&self) -> &str;
}