use std;
use std::fs::File;
use std::io::prelude::*;
use toml_edit::Document;

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

#[derive(Debug, Clone)]
pub struct HostConfig {
    pub host: String,
    pub deploy_path: String,
    pub keep_releases: i8,
    pub repo_url: String,
}

pub fn parse_config_file() -> GenResult<HostConfig> {
    let mut file = File::open("Deployer.toml")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let doc = contents.parse::<Document>()?;
    let host_table = doc["staging"].as_table().unwrap();

    let host = String::from(host_table.get("host").unwrap().as_str().unwrap());
    let deploy_path = String::from(host_table.get("deploy-path").unwrap().as_str().unwrap());
    let keep_releases = host_table.get("keep-releases").unwrap().as_integer().unwrap() as i8;
    let repo_url = String::from(host_table.get("repo-url").unwrap().as_str().unwrap());

    Ok(HostConfig { host, deploy_path, keep_releases, repo_url})
}