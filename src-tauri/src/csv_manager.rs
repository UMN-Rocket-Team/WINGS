use std::{fs::{self, File}, path::Path};
use anyhow::{bail, Error};
use csv::Writer;

use crate::models::packet::Packet;

pub struct CSVManager {
    csv_writer: Writer<File>,
    byte_writer: String
}

///The default write path of the CSVManager is ./packetslog.csv, the program should crash if it can't write to that directory
impl Default for CSVManager {
    fn default() -> Self {
        Self {
            csv_writer: csv::Writer::from_path(Path::new("../packetslog.csv")).unwrap(),
            byte_writer: "../rawbytes.wings".to_owned()
        }
    }
}

impl CSVManager{

    ///Sets the filepath that the CSVManager writes to. returns an error if it cant write to that path
    pub fn set_write(&mut self, path: String)-> Result<(),Error> {
        match csv::Writer::from_path(Path::new(&path)) {
            Err(err) => bail!("unable to load from path {}, error: {}", path, err),
            Ok(writer) => {self.csv_writer = writer; Ok(())},
        }  
    }
    
    pub fn write_packet(&mut self, packet: Packet)-> Result<(),Error> {
        match self.csv_writer.serialize(packet.field_data){
            Err(err) => bail!("Unable to write packet and got error: {}", err),
            Ok(_) => {
                match self.csv_writer.flush() {
                    Err(err) => bail!("Unable to flush packet writer and got error: {}", err),
                    Ok(ok) => Ok(ok),
                }
            },
        }  
    }

    pub fn write_bytes(&mut self, mut data: Vec<u8>)-> Result<(),Error> {
        let mut old_data = vec![];
        match fs::read(self.byte_writer.clone()){
            Err(_) => {},
            Ok(ok) => {old_data = ok},
        }  
        old_data.append(&mut data);
        match fs::write(self.byte_writer.clone(), old_data){
            Err(err) => bail!("Unable to write packet and got error:{}", err),
            Ok(ok) => {Ok(ok)},
        }  
    }
}