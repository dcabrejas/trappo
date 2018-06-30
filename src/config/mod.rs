use std::fs::File;
use std::io::prelude::*;
use toml_edit::{Document, Table};

pub mod error;

#[derive(Debug, Clone)]
pub struct HostConfig {
    pub host: String,
    pub deploy_path: String,
    pub keep_releases: i8,
    pub repo_url: String,
    pub link_files: Vec<String>,
    pub link_dirs: Vec<String>
}

pub fn parse_config_file(file_name: &str, stage: &str) -> Result<HostConfig, error::ParseError> {
    let mut file = File::open(file_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let doc = contents.parse::<Document>().unwrap();
    let host_table = match doc[stage].as_table() {
        Some(host_table) => host_table,
        None => {
            return Err(error::ParseError::new("Error parsing host table".into()))
        }
    };

    Ok(
        HostConfig {
            host: get_from_table_as_string("host", &host_table)?,
            deploy_path: get_from_table_as_string("deploy-path", &host_table)?,
            keep_releases: get_from_table_as_integer("keep-releases", &host_table)? as i8,
            repo_url: get_from_table_as_string("repo-url", &host_table)?,
            link_files: get_from_table_as_string_vector("linked_files", &host_table)?,
            link_dirs: get_from_table_as_string_vector("linked_dirs", &host_table)?
        }
    )
}

fn get_from_table_as_string(key: &str, table: &Table) -> Result<String, error::ParseError> {
    let value = match table.get(key) {
        Some(value) => match value.as_str() {
            Some(value) => value,
            None => {
                let error = format!("Error parsing {} from config file", key);
                return Err(error::ParseError::new(error))
            }
        },
        None => {
            let error = format!("{} value not found in config file", key);
            return Err(error::ParseError::new(error))
        }
    };

    Ok(value.into())
}

fn get_from_table_as_integer(key: &str, table: &Table) -> Result<i64, error::ParseError> {
    let value = match table.get(key) {
        Some(value) => match value.as_integer() {
            Some(value) => value,
            None => {
                let error = format!("Error parsing {} from config file", key);
                return Err(error::ParseError::new(error))
            }
        },
        None => {
            let error = format!("{} value not found in config file", key);
            return Err(error::ParseError::new(error))
        }
    };

    Ok(value)
}

fn get_from_table_as_string_vector(key: &str, table: &Table) -> Result<Vec<String>, error::ParseError> {
    let value = match table.get(key) {
        Some(value) => match value.as_array() {
            Some(value) => {
                value.iter().map(|value| value.as_str().unwrap().to_string()).collect()
            },
            None => {
                let error = format!("Error parsing {} from config file", key);
                return Err(error::ParseError::new(error))
            }
        },
        None => {
            let error = format!("{} value not found in config file", key);
            return Err(error::ParseError::new(error))
        }
    };

    Ok(value)
}
