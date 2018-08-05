use std::fs::File;
use std::io::prelude::*;
use toml_edit::*;
use std::collections::HashMap;

pub mod error;

use self::error::*;

#[derive(Debug, Clone)]
pub struct HostConfig {
    pub host: String,
    pub deploy_path: String,
    pub keep_releases: i8,
    pub repo_url: String,
    pub link_files: Vec<String>,
    pub link_dirs: Vec<String>
}

pub fn parse_config_file(file_name: &str, stage: &str) -> Result<HostConfig, ConfigError> {
    let mut file = File::open(file_name)
        .map_err(|_| ConfigError::NotFound(stage.into()))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| ConfigError::IoError)?;

    let doc = contents.parse::<Document>().unwrap();

    //save global table (Option) for overriden values later
    let global_table = doc["global"].as_table();

    let host_table = match doc[stage].as_table() {
        Some(table) => table,
        None => {
            return Err(ConfigError::BadStage(stage.into()))
        }
    };

    let mut config_map = HashMap::new();

    //populate hash map with the stage config items
    for (key, value) in host_table.iter() {
        config_map.insert(key, value);
    };

    //if global table is present, override stage items with global items
    if let Some(globals) = global_table {
        for (key, value) in globals.iter() {
            config_map.insert(key, value);
        };
    }

    Ok(
        HostConfig {
            host: parse_string("host", &config_map)?,
            deploy_path: parse_string("deploy-path", &config_map)?,
            //keep_releases defaults to 3 if not present
            keep_releases: parse_integer("keep-releases", &config_map).unwrap_or(3 as i64) as i8,
            repo_url: parse_string("repo-url", &config_map)?,
            link_files: parse_string_vector("linked-files", &config_map).unwrap_or(Vec::new()),
            link_dirs: parse_string_vector("linked-dirs", &config_map).unwrap_or(Vec::new())
        }
    )
}

fn parse_integer(key: &str, hash_map: &HashMap<&str, &Item>) -> Result<i64, ConfigError> {
    let value = match hash_map.get(key) {
        Some(value) => match value.as_integer() {
            Some(value) => value,
            None => return Err(ConfigError::BadType(key.into(), "integer".into()))
        },
        None => return Err(ConfigError::MissingField(key.into()))
    };

    Ok(value)
}

fn parse_string(key: &str, hash_map: &HashMap<&str, &Item>) -> Result<String, ConfigError> {
    let value = match hash_map.get(key) {
        Some(value) => match value.as_str() {
            Some(value) => value,
            None => return Err(ConfigError::BadType(key.into(), "string".into()))
        },
        None => return Err(ConfigError::MissingField(key.into()))
    };

    Ok(value.into())
}

fn parse_string_vector(key: &str, hash_map: &HashMap<&str, &Item>) -> Result<Vec<String>, ConfigError> {
    let value = match hash_map.get(key) {
        Some(value) => match value.as_array() {
            Some(value) => value.iter().map(|value| value.as_str().unwrap().to_string()).collect(),
            None => return Err(ConfigError::BadType(key.into(), "array".into()))
        },
        None => return Err(ConfigError::MissingField(key.into()))
    };

    Ok(value)
}
