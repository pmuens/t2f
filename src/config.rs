use std::io::prelude::*;
use std::fs::File;

use serde_json;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Mapping {
    pub regex: String,
    pub event: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventGateway {
    pub url: String,
    pub access_key: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub file_path: String,
    pub mappings: Vec<Mapping>,
    pub event_gateway: EventGateway
}

pub fn parse(ref file_path: &String) -> Config {
    let mut file = File::open(file_path).expect("config file not found");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("config file could not be parsed");

    serde_json::from_str(&content).unwrap()
}
