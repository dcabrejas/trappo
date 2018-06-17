use std::fs::File;
use std::io::prelude::*;
use toml_edit::Document;

#[derive(Debug, Clone)]
pub struct HostConfig {
    pub host: String,
    pub deploy_path: String,
    pub keep_releases: i8
}

pub fn parse_config_file() -> Result<HostConfig, &'static str> {
    let mut file = File::open("Deployer.toml")
        .expect("Failed to read configuration file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let doc = contents.parse::<Document>().expect("invalid doc");
    let host_table = doc["staging"].as_table().unwrap();

    let host = String::from(host_table.get("host").unwrap().as_str().unwrap());
    let deploy_path = String::from(host_table.get("deploy-path").unwrap().as_str().unwrap());
    let keep_releases = host_table.get("keep-releases").unwrap().as_integer().unwrap() as i8;

    Ok(HostConfig { host, deploy_path, keep_releases})
}