use std::fs::{self, read_to_string};

use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};

use crate::file_handling;

///the config that will be generated if no config is present
const DEFAULT_CONFIG: ConfigStruct = ConfigStruct {
    default_baud: 56700
};
#[derive(Deserialize, Serialize)]
pub struct ConfigStruct{
    default_baud: u32
}

///Default will either be what is already in the config file, or the DEFAULT_CONFIG const
impl Default for ConfigStruct{
    fn default() -> Self {
        match read_config(){
            Ok(config_struct) => return config_struct,
            Err(err) => {
                eprintln!("{}", err.context("Failed to load from config file, attempting new config generation"));
                let result = make_config();
                match result{
                    Ok(_) => {
                        return DEFAULT_CONFIG;
                    },
                    Err(error) => {
                        eprintln!("{}", error.context("Failed config file generation"));
                        return DEFAULT_CONFIG;
                    },
                }
            },
        }
    }
}

fn read_config() -> Result<ConfigStruct,Error>{
    let mut path_buf = tauri::api::path::data_dir().expect("no data dir found on this system");
    path_buf.push(file_handling::BASE_DIRECTORY);
    path_buf.push("config");
    path_buf.set_extension("json");
    let file_string = read_to_string(path_buf.as_path())
        .with_context(|| "Failed to read config to string")?;
    let config_struct = serde_json::from_str(&file_string)
        .with_context(|| "Failed to parse config to struct, most likely bad formatting")?;
    Ok(config_struct)
}

fn make_config() -> Result<(),Error>{
    let mut path_buf = tauri::api::path::data_dir().expect("no data dir found on this system");
    path_buf.push(file_handling::BASE_DIRECTORY);
    path_buf.push("config");
    path_buf.set_extension("json");
    fs::write(path_buf.as_path(), serde_json::to_string(&DEFAULT_CONFIG)?)
        .with_context(|| "Unable to write default config to config file")?;
    Ok(())
}
impl ConfigStruct{
    
}