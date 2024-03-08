use anyhow::{bail, Error};
use csv::{Reader, StringRecord, Writer};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::models::packet::Packet;
/// Acts as a general data structure to store all files that the ground station is currently interacting with
pub struct FileHandler {
    csv_writer: Writer<File>,
    csv_reader: Reader<File>,
    byte_writer: File,
}

/// The default write path of the CSVManager is ./packetslog.csv, the program should crash if it can't write to that directory
impl Default for FileHandler {
    fn default() -> Self {
        let mut taken = true;
        let mut i = 0;
        while taken{
            i = i + 1;
            taken = taken && Path::new(&format!("../logs/packetslog{i}.csv")).exists();
            taken = taken && Path::new(&format!("../logs/rawbytes{i}.wings")).exists();
        }
        //add logic to set up .wings
        Self {
            csv_writer: csv::Writer::from_path(Path::new(&format!("../logs/packetslog{i}.csv"))).unwrap(),
            csv_reader: csv::ReaderBuilder::new()
                .has_headers(false)
                .from_path("../input.csv").unwrap(),
            byte_writer: fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(&format!("../logs/rawbytes{i}.wings"))
                .unwrap(),
        }
    }
}

impl FileHandler {
    /// Sets the filepath that the FileHandler writes csvs to. returns an error if it can't write to that path
    ///
    /// path: the path that we want to write csv 
    pub fn set_write(&mut self, path: String) -> Result<(), Error> {
        match csv::Writer::from_path(Path::new(&path)) {
            Err(err) => bail!("unable to load from path {}, error: {}", path, err),
            Ok(writer) => {
                self.csv_writer = writer;
                Ok(())
            }
        }
    }

    /// Write a packet to the csv currently loaded
    ///
    /// packet: the packet which has the data we want to write
    pub fn write_packet(&mut self, packet: Packet) -> Result<(), Error> {
        match self.csv_writer.serialize(packet.field_data) {
            Err(err) => bail!("Unable to write packet and got error: {}", err),
            Ok(_) => match self.csv_writer.flush() {
                Err(err) => bail!("Unable to flush packet writer and got error: {}", err),
                Ok(ok) => Ok(ok),
            },
        }
    }

    
    /// Sets the filepath that the FileHandler reads csvs from. returns an error if it can't write to that path
    ///
    /// path: the path that we want to read csv data from
    pub fn set_read(&mut self, path: String) -> Result<(), Error> {
        match csv::ReaderBuilder::new().has_headers(false).from_path(Path::new(&path)) {
            Err(err) => bail!("unable to load from path {}, error: {}", path, err),
            Ok(reader) => {
                self.csv_reader = reader;
                Ok(())
            }
        }
    }

    /// Read a packet from the csv currently loaded
    pub fn read_packet(&mut self) -> Result<StringRecord, Error> {
        match self.csv_reader.records().next() {
            None => bail!("nothing read from csv"),
            Some(ok) => match ok {
                Err(err) => bail!("Failed to read from csv: {}", err),
                Ok(ok) => Ok(ok),
            },
        }
    }

    /// Write a packet to the byte file currently loaded
    ///
    /// bytes: raw bytes to write into the file
    pub fn write_bytes(&mut self, data: Vec<u8>) -> Result<usize, Error> {
        match self.byte_writer.write(&data) {
            Err(err) => bail!("Unable to write packet and got error:{}", err),
            Ok(ok) => Ok(ok),
        }
    }
}
