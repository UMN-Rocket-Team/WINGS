use std::{fs::File, path::Path};
use anyhow::{bail, Error};
use csv::Writer;

use crate::models::packet::Packet;

pub struct CSVManager {
    writer: Writer<File>,
}

///The default write path of the CSVManager is ./packetslog.csv, the program should crash if it can't write to that directory
impl Default for CSVManager {
    fn default() -> Self {
        Self {
            writer: csv::Writer::from_path(Path::new("../packetslog.csv")).unwrap(),
        }
    }
}

impl CSVManager{

    ///Sets the filepath that the CSVManager writes to. returns an error if it cant write to that path
    pub fn set_write(&mut self, path: String)-> Result<(),Error> {
        match csv::Writer::from_path(Path::new(&path)) {
            Err(err) => bail!("unable to load from path {}, error: {}", path, err),
            Ok(writer) => {self.writer = writer; Ok(())},
        }  
    }
    
    pub fn write_packet(&mut self, packet: Packet)-> Result<(),Error> {
        match self.writer.serialize(packet.field_data){
            Err(_) => bail!("Unable to write packet"),
            Ok(_) => {
                match self.writer.flush() {
                    Err(_) => bail!("Unable to flush packet writer"),
                    Ok(_) => Ok(()),
                }
            },
        }  
    }
}