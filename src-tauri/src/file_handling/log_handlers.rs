use anyhow::{bail, Error};
use chrono::Utc;
use csv::{Reader, StringRecord, Writer};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{models::packet::Packet, packet_structure_manager::PacketStructureManager};
pub const BASE_DIRECTORY: &str = "Wings_data_logs";
const DAY_FORMAT: &str = "%F";
const TIME_FORMAT: &str = "%H_%M_%S";
const LOG_TIME_FORMAT: &str = "%a_%F_%H_%M_%S";
/// Acts as a general data structure to store all files that the ground station is currently interacting with
pub struct LogHandler {
    csv_writers: Vec<PacketWriter>,//a list of all csv writers within the FileHandler(one for each packet)
    csv_reader: Option<Reader<File>>,
    byte_writer: File,
    base_path: PathBuf,
    safety_iterator: u8,

}

struct PacketWriter{
    writer: Writer<File>,
    index: usize
}

impl Default for LogHandler {
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
        let mut path_buf = tauri::api::path::data_dir().expect("no data dir found on this system");
        println!("{:#?}", path_buf);
        path_buf.push(BASE_DIRECTORY);
        let _ = fs::create_dir(path_buf.as_path());
        let time = Utc::now();
        path_buf.push(&format!("{}",time.format(DAY_FORMAT)));
        let _ = fs::create_dir(path_buf.as_path());
        path_buf.push(&format!("{}",time.format(TIME_FORMAT)));
        let _ = fs::create_dir(path_buf.as_path());
        let general_directory = path_buf.clone();
        path_buf.push(&format!("rawbytes_log_{}",time.format(LOG_TIME_FORMAT)));
        path_buf.set_extension("wings");
        Self {
            csv_writers: vec![],
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
                    .open(path_buf.as_path()).unwrap(),
            safety_iterator: 1,
            base_path: general_directory,

        }
    }
}

impl LogHandler {

    /// Write a packet to the csv currently loaded
    ///
    /// Note: this function flushes the writer buffer with every call, 
    /// meaning that we aren't making use of the csv_writer internal buffer.
    /// This is safer when it comes to saving as much data as possible, but might possibly slow down the groundstation
    /// 
    /// # Errors
    /// 
    /// produces an error if unsuccessful
    pub fn write_packet(&mut self, mut packet: Packet, packet_structure_manager: &mut PacketStructureManager) -> Result<(), Error> {
        let csv_writer;
        match self.find_writer_index(packet.structure_id){
            Some(index) => csv_writer = &mut self.csv_writers[index],
            None => {
                let mut field_names = vec![];
                let mut path_buf = self.base_path.clone();
                let _ = fs::create_dir(path_buf.as_path());
                match packet_structure_manager.get_packet_structure(packet.structure_id){
                    Ok(packet_structure) => {
                        path_buf.push(format!("packet_log_{}", packet_structure.name)); 
                        for field in &packet_structure.fields{
                            field_names.push(field.name.to_owned());
                        }
                    },
                    Err(_) => {path_buf.push(format!("packet_log_{}", packet.structure_id))},
                }
                path_buf.set_extension("csv");
                self.csv_writers.push(
                    PacketWriter{ writer: csv::Writer::from_path(path_buf.as_path())?, index: packet.structure_id}
                );
                let iter = self.csv_writers.len() - 1;
                csv_writer = &mut self.csv_writers[iter];
                let _ = csv_writer.writer.serialize(field_names);//dont care if this actually gets written, headers can get reverse engineered
            },
        }
        
        match csv_writer.writer.serialize(packet.field_data) {
            Err(err) => {
                _ = csv_writer.writer.flush(); //attempt to flush, we dont handle the result since we are already failing anyways
                packet.field_data = Default::default();
                self.safety_iterator += 1;
                let mut path_buf = self.base_path.clone();
                path_buf.push(format!("error_log_{}_{}", packet.structure_id, self.safety_iterator));
                path_buf.set_extension("csv");

                csv_writer.writer = csv::Writer::from_path(path_buf.as_path())?;
                bail!("Unable to write packet and got error: {}", err);
            },
            Ok(_) => match csv_writer.writer.flush() {
                Err(err) => {
                    bail!("Unable to flush packet writer and got error: {}", err)
                },
                
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

    fn find_writer_index(&mut self, id: usize) -> Option<usize>{
        self.csv_writers.iter().position(|r| r.index == id)
    }
}

#[cfg(test)]
mod tests {
    //TODO
    //write a systems test to check all functions within file_handling
}
