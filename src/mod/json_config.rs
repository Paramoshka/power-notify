use std::fs::File;
use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Config {
    token: String
}

pub async fn read_config() -> Config {
    let open_file = File::open("config.json");
    match open_file {
        Ok(_) => {}
        Err(_) => {}
    }
    let file: Result<Config, Err()> = serde_json::from_str("");
    match file {
        Ok(config) => {return config},
        Err(err) => panic!("Problem opening the file: {:?}", err),
    };
}