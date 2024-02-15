use anyhow::{bail, Error};
use csv::{Reader, StringRecord, Writer};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::models::packet::Packet;

pub struct FileHandler {
    csv_writer: Writer<File>,
    csv_reader: Reader<File>,
    byte_writer: File,
}

///The default write path of the CSVManager is ./packetslog.csv, the program should crash if it can't write to that directory
impl Default for FileHandler {
    fn default() -> Self {
        //add logic to set up .wings
        Self {
            csv_writer: csv::Writer::from_path(Path::new("../packetslog.csv")).unwrap(),
            csv_reader: csv::ReaderBuilder::new()
                .has_headers(false)
                .from_path("../input.csv").unwrap(),
            byte_writer: fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("../rawbytes.wings")
                .unwrap(),
        }
    }
}

impl FileHandler {
    ///Sets the filepath that the CSVManager writes to. returns an error if it cant write to that path
    pub fn set_write(&mut self, path: String) -> Result<(), Error> {
        match csv::Writer::from_path(Path::new(&path)) {
            Err(err) => bail!("unable to load from path {}, error: {}", path, err),
            Ok(writer) => {
                self.csv_writer = writer;
                Ok(())
            }
        }
    }

    pub fn write_packet(&mut self, packet: Packet) -> Result<(), Error> {
        match self.csv_writer.serialize(packet.field_data) {
            Err(err) => bail!("Unable to write packet and got error: {}", err),
            Ok(_) => match self.csv_writer.flush() {
                Err(err) => bail!("Unable to flush packet writer and got error: {}", err),
                Ok(ok) => Ok(ok),
            },
        }
    }
    pub fn read_packet(&mut self) -> Result<StringRecord, Error> {
        match self.csv_reader.records().next() {
            None => bail!("nothing read from csv"),
            Some(ok) => match ok {
                Err(err) => bail!("Failed to read from csv: {}", err),
                Ok(ok) => Ok(ok),
            },
        }
    }

    pub fn write_bytes(&mut self, data: Vec<u8>) -> Result<usize, Error> {
        match self.byte_writer.write(&data) {
            Err(err) => bail!("Unable to write packet and got error:{}", err),
            Ok(ok) => Ok(ok),
        }
    }
}
