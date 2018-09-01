pub mod error;

use std::io::{ErrorKind, Read};
use std::fs::File;
use self::error::{StepConfigError, StepConfigError::*};
use toml_edit::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum StepPosition {
    Before,
    After
}

#[derive(Debug, Clone)]
pub struct StepConfig {
    pub name: String,
    pub comand: String,
    pub position: StepPosition,
    pub ref_step: String
}

type Steps = HashMap<String, StepConfig>;

pub fn parse_steps_config_file(file_name: &str, stage: &str) -> Result<Vec<StepConfig>, StepConfigError> {
    let file = File::open(file_name);

    //return empty vector if steps file is not present.
    if file.is_err() {
        match file.unwrap_err().kind() {
            ErrorKind::NotFound => return Ok(Vec::new()),
            _ => return Err(StepConfigError::IoError)
        }
    }

    let mut file = file.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|_| StepConfigError::IoError)?;

    let doc = contents.parse::<Document>().unwrap();

    let mut steps_map: HashMap<String, StepConfig> = HashMap::new();
    parse_steps_and_add_to_hashmap(&doc, stage, &mut steps_map)?;
    parse_steps_and_add_to_hashmap(&doc, "global", &mut steps_map)?;

    let steps_collection: Vec<StepConfig> = steps_map.drain().map(|(_, k)| { k }).collect();
    Ok(steps_collection)
}

fn parse_steps_and_add_to_hashmap(doc: &Document, stage: &str, collection: &mut Steps)
    -> Result<(), StepConfigError> {

    if let Some(stage_steps) = doc[stage]["steps"].as_array_of_tables() {
        for table in stage_steps.iter() {
            let step = parse_step_table(table)?;
            collection.insert(step.name.to_owned(), step);
        };
    }

    Ok(())
}

fn parse_step_table(table: &Table) -> Result<StepConfig, StepConfigError> {
    let name = table.get("name").ok_or(MissingField("name".into()))?.as_str()
        .ok_or(BadType("name".into(), "string".into()))?.to_string();

    let comand = table.get("command").ok_or(MissingField("command".into()))?.as_str()
        .ok_or(BadType("command".into(), "string".into()))?.to_string();

    if !table.contains_key("before") && !table.contains_key("after") {
        return Err(AmbiguousPosition)
    };

    let (position, ref_step) = if table.contains_key("before") {
        let ref_step = table.get("before").unwrap().as_str().ok_or(BadType("before".into(), "string".into()))?.to_string();
        (StepPosition::Before, ref_step)
    } else if table.contains_key("after") {
        let ref_step = table.get("after").unwrap().as_str().ok_or(BadType("after".into(), "string".into()))?.to_string();
        (StepPosition::After, ref_step)
    } else {
        return Err(MissingPosition);
    };

    Ok(
        StepConfig {
            name,
            comand,
            position,
            ref_step
        }
    )
}
