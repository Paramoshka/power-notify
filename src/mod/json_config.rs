use std::fs;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Config {
    pub token: String,
    pub admin_id: String,
    pub chat_id: String
}

pub async fn read_config() -> Config {
    let data = fs::read_to_string("config.json").expect("Unable to read file");
    let file  = serde_json::from_str(&data).expect("file not correct");
    return file;
}