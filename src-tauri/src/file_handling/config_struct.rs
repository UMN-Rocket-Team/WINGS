use std::fs::{self, read_to_string};

use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};

use crate::{packet_structure_manager::PacketStructureManager, state::packet_structure_manager_state::default_packet_structure_manager};
use crate::file_handling::log_handlers::BASE_DIRECTORY;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ConfigStruct{
    pub default_baud: u32,
    pub packet_structure_manager: PacketStructureManager
}


fn hard_coded_config() -> ConfigStruct {
    ConfigStruct{
        default_baud: 56700,
        packet_structure_manager: default_packet_structure_manager(),
    }
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
                        return hard_coded_config();
                    },
                    Err(error) => {
                        eprintln!("{}", error.context("Failed config file generation"));
                        return hard_coded_config();
                    },
                }
            },
        }
    }
}

fn read_config() -> Result<ConfigStruct,Error>{
    let mut path_buf = tauri::api::path::data_dir().expect("no data dir found on this system");
    path_buf.push(BASE_DIRECTORY);
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
    path_buf.push(BASE_DIRECTORY);
    path_buf.push("config");
    path_buf.set_extension("json");
    fs::write(path_buf.as_path(), serde_json::to_string(&hard_coded_config())?)
        .with_context(|| "Unable to write default config to config file")?;
    Ok(())
}
impl ConfigStruct{
    
}

#[cfg(test)]
mod tests {
    use super::*; // lets the unit tests use everything in this file

    #[test]
    #[ignore]
    fn print_config_as_read() {
        println!("{:#?}", ConfigStruct::default());
    }
}