use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use wings_packet_parser::{
    communication_manager::DeviceName,
    models::packet_structure::{PacketCRC, PacketDelimiter, PacketField, PacketStructure},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    port: u16,
    radio_options: Option<RadioOptions>,
    packet_structures: Vec<ConfigPacketStructure>,
}

impl<'a> Config {
    pub fn load_or_default(path: &'a PathBuf) -> Config {
        match Self::get_config(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "Could not read config due to the following error\n{}\nBuilding from default",
                    e.to_string()
                );
                let config = Self {
                    port: 3001u16,
                    radio_options: None,
                    packet_structures: vec![],
                };
                match e {
                    ConfigError::NoPath(_e, path_buf) => {
                        let _ = config.save_config(&path_buf);
                    }
                    _ => {}
                }
                config
            }
        }
    }
    fn get_config(path: &'a PathBuf) -> Result<Config, ConfigError<'a>> {
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => return Err(ConfigError::NoPath(e, path)),
        };
        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(_u) => {}
            Err(e) => return Err(ConfigError::File(e, path)),
        }
        match toml::from_str::<Config>(buf.as_str()) {
            Ok(config) => Ok(config),
            Err(e) => Err(ConfigError::Deserialization(e)),
        }
    }

    pub fn save_config(&'a self, path: &'a PathBuf) -> Result<(), ConfigError<'a>> {
        let mut file = match File::create(&path) {
            Ok(f) => f,
            Err(e) => return Err(ConfigError::NoPath(e, &path)),
        };
        let config_string = match toml::ser::to_string::<Config>(self) {
            Ok(str) => str,
            Err(e) => return Err(ConfigError::Serialization(e)),
        };
        match file.write_all(config_string.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(ConfigError::File(e, &path)),
        }
    }
}

#[derive(Debug)]
pub enum ConfigError<'a> {
    File(std::io::Error, &'a PathBuf),
    Deserialization(toml::de::Error),
    NoPath(std::io::Error, &'a PathBuf),
    Serialization(toml::ser::Error),
}

impl<'a> std::fmt::Display for ConfigError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConfigError::File(e, p) => format!("{}: {}", e.to_string(), p.to_str().unwrap()),
                ConfigError::Deserialization(e) => e.to_string(),
                ConfigError::NoPath(e, p) => format!("{}: {}", e.to_string(), p.to_str().unwrap()),
                ConfigError::Serialization(e) => e.to_string(),
            }
        )
    }
}

impl<'a> std::error::Error for ConfigError<'a> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RadioOptions {
    transmitter: Option<DeviceName>,
    reciever: Option<DeviceName>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigPacketStructure {
    pub id: usize,
    pub name: String,
    pub byte_defined: Option<bool>, //marks wether or not a packet describes its full binary layout
    pub fields: Vec<PacketField>,
    pub delimiters: Vec<PacketDelimiter>,
    pub packet_crc: Vec<PacketCRC>,
    pub size: Option<usize>,
}

impl ConfigPacketStructure {
    pub fn to_packet_structure(&self) -> PacketStructure {
        PacketStructure {
            id: self.id,
            name: self.name.clone(),
            byte_defined: self.byte_defined.unwrap_or_default(),
            fields: self.fields.clone(),
            delimiters: self.delimiters.clone(),
            packet_crc: self.packet_crc.clone(),
            size: self.size,
        }
    }
}

/* EXAMPLE CONFIG FILE
port = 3001
radio_options = {
    transmitter = {
        name = "transmitter_name"
        value = "transmitter_value"
        product_name = "transmitter_product_name"
        manufacturer_name = "transmitter_manufacturer_name"
    }
    reciever = {
        name = "reciever_name"
        value = "reciever_value"
        product_name = "reciever_product_name"
        manufacturer_name = "reciever_manufacturer_name"
    }
}
packet_structures = [
    {
        components = [
            {
                data = {
                    identifier = 11ba5eba
                    index = 0,
                    name = start_delim
                    offsetInPacket 0
                    type = Delimiter
                },
            {
                data =
                    index = 0
                    name = "Timestamp"
                    offsetInPacket = 4
                    type = "Unsigned Integer"
                    type = "Field"
                {,
            {
                data = {()

                    index = 1
                    name = "pkt_type"
                    offsetInPacket = 8
                    type = "Unsigned Byte"
                    type = "Field"
                },
            {
                data = {
                    index = 2
                    name = "state"
                    offsetInPacket = 9
                    type = "Unsigned Byte"()
                    type = "Field"
                },
            {
                data = {
                    index = 3
                    name = "pkt_len"
                    offsetInPacket = 10
                    type = "Unsigned Short"
                    type = "Field"
                },
            {
                data = {
                    index = 4
                    name = "center_port"
                    offsetInPacket = 12
                    type = "Float"
                    type = "Field"
                },
            {
                data = {
                    index = 5
                    name = "static_port"
                    offsetInPacket = 16
                    type = "Float"
                    type = "Field"
                },
            {
                data = {
                    identifier = "1eab11ca"
                    index = 1
                    name = "end_delim"
                    offsetInPacket = 20
                    type = "Delimiter"
                    id = 18
                    name = "UFC_NEW_PITOT_CENTER"
                },
            {
        ]
    }
]
 */
