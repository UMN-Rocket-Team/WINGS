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
    csv_writer:Writer<File>,
    csv_reader: Option<Reader<File>>,
    byte_writer: File,
}

impl Default for FileHandler {
    /// Makes a default configuration for the file handler
    /// 
    /// the handler will first generate a logs directory in parallel to the execution directory
    /// 
    /// The handler will then find unused .csv, and .wings file names within the ./logs folder
    /// These will be the save file names.
    /// 
    /// the file will also look for an input.csv within both the execution directory, and the directory that contains it.
    /// 
    /// # Panics
    /// 
    /// this program will panic if it is unable to generate valid write files, this is done to prevent wings from starting without a log to save to
    fn default() -> Self {
        let _ = fs::create_dir("../logs");
        let mut taken = true;
        let mut i = 0;
        while taken{
            i = i + 1;
            taken = taken && Path::new(&format!("../logs/packetslog{i}.csv")).exists();
            taken = taken && Path::new(&format!("../logs/rawbytes{i}.wings")).exists();
        }
        Self {
            csv_writer: csv::Writer::from_path(Path::new(&format!("../logs/packetslog{i}.csv"))).unwrap(),
            csv_reader: 
                match csv::ReaderBuilder::new()
                    .has_headers(false)
                    .from_path("../input.csv"){
                        Ok(ok) => Some(ok),
                        Err(_) => {
                            match csv::ReaderBuilder::new()
                                .has_headers(false)
                                .from_path("./input.csv"){
                                    Ok(ok) => Some(ok),
                                    Err(_) => None,
                            }
                        },
                },
            byte_writer: 
                fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&format!("../logs/rawbytes{i}.wings")).unwrap()
        }
    }
}

impl FileHandler {
    /// Sets the filepath that the FileHandler writes csv's to. returns an error if it can't write to that path
    ///
    /// # Errors
    /// 
    /// returns an error if something goes wrong when loading the file from that directory (the old csv write directory will remain)
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
    /// Note: this function flushes the writer  buffer with every call, 
    /// meaning that we aren't making use of the csv_writer internal buffer.
    /// This is safer when it comes to saving as much data as possible, but might possibly slow down the groundstation
    /// 
    /// # Errors
    /// 
    /// produces an error if unsuccessful
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
    /// This assumes that the csv has no header data
    /// 
    /// # Errors
    /// 
    /// returns an error if something goes wrong when loading the file from that directory (the old csv reader directory will remain)
    pub fn set_read(&mut self, path: String) -> Result<(), Error> {
        match csv::ReaderBuilder::new().has_headers(false).from_path(Path::new(&path)) {
            Err(err) => bail!("unable to load from path {}, error: {}", path, err),
            Ok(reader) => {
                self.csv_reader = Some(reader);
                Ok(())
            }
        }
    }

    /// Read a packet from the csv currently loaded
    /// 
    /// # Errors
    /// 
    /// Returns an error if
    /// - the read fails, 
    /// - there is no input,
    /// - the reader has reached the end of a csv
    pub fn read_packet(&mut self) -> Result<StringRecord, Error> {
        if self.csv_reader.is_some(){
            match self.csv_reader.as_mut().unwrap().records().next() {
                None => bail!("nothing read from csv"),
                Some(ok) => match ok {
                    Err(err) => bail!("Failed to read from csv: {}", err),
                    Ok(ok) => Ok(ok),
                },
            }
        }
        else{
            bail!("No csv read directory")
        }
    }

    /// Write a packet to the byte file currently loaded
    /// 
    /// # Errors
    /// 
    /// produces an error if unsuccessful
    pub fn write_bytes(&mut self, data: Vec<u8>) -> Result<usize, Error> {
        match self.byte_writer.write(&data) {
            Err(err) => bail!("Unable to write packet and got error:{}", err),
            Ok(ok) => Ok(ok),
        }
    }
}
