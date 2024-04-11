use anyhow::{bail, Error};
use chrono::{DateTime, Utc};
use csv::{Reader, StringRecord, Writer};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use crate::models::packet::Packet;

/// Acts as a general data structure to store all files that the ground station is currently interacting with
pub struct FileHandler {
    csv_writers: Vec<PacketWriter>,
    csv_reader: Option<Reader<File>>,
    byte_writer: File,
    time: DateTime<Utc>,
    safety_iterator: u8,

}

struct PacketWriter{
    writer: Writer<File>,
    index: usize
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
        let mut path_buf = tauri::api::path::data_dir().expect("no data dir found on this system");
        println!("{:#?}", path_buf);
        path_buf.push("wings_logs");
        let _ = fs::create_dir(path_buf.as_path());
        let time = Utc::now();
        path_buf.push(&format!("{}",time.format("%Y_%m_%d_%H_%M_%S")));
        let _ = fs::create_dir(path_buf.as_path());
        path_buf.push(&format!("rawbytes{}",time.format("%Y_%m_%d_%H_%M_%S")));
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
            time: time,

        }
    }
}

impl FileHandler {

    /// Write a packet to the csv currently loaded
    ///
    /// Note: this function flushes the writer  buffer with every call, 
    /// meaning that we aren't making use of the csv_writer internal buffer.
    /// This is safer when it comes to saving as much data as possible, but might possibly slow down the groundstation
    /// 
    /// # Errors
    /// 
    /// produces an error if unsuccessful
    pub fn write_packet(&mut self, mut packet: Packet) -> Result<(), Error> {
        let csv_writer;
        match self.find_writer_index(packet.structure_id){
            Some(index) => csv_writer = &mut self.csv_writers[index],
            None => {
                let mut path_buf: PathBuf;
                match tauri::api::path::data_dir(){
                    Some(data) => path_buf = data,
                    None => bail!("Unable to get data path"),
                }
                path_buf.push("wings_logs");
                path_buf.push(format!("{}",self.time.format("%Y_%m_%d_%H_%M_%S")));
                let _ = fs::create_dir(path_buf.as_path());
                path_buf.push(format!("packetslog_{}", packet.structure_id));
                path_buf.set_extension("csv");
                self.csv_writers.push(
                    PacketWriter{ writer: csv::Writer::from_path(path_buf.as_path())?, index: packet.structure_id}
                );
                let iter = self.csv_writers.len() - 1;
                csv_writer = &mut self.csv_writers[iter];
            },
        }
        
        match csv_writer.writer.serialize(packet.field_data) {
            Err(err) => {
                _ = csv_writer.writer.flush(); //attempt to flush, we dont handle the result since we are already failing anyways
                packet.field_data = Default::default();
                self.safety_iterator += 1;
                let mut path_buf: PathBuf;
                match tauri::api::path::data_dir(){
                    Some(data) => path_buf = data,
                    None => bail!("FATAL ERROR, Unable to write packet and got error, and unable to get a new directory: {}", err),
                }
                path_buf.push("wings_logs");
                path_buf.push(format!("{}",self.time.format("%Y_%m_%d_%H_%M_%S")));
                path_buf.push(format!("packetslog_{}_{}", packet.structure_id, self.safety_iterator));
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

    fn find_writer_index(&mut self, index: usize) -> Option<usize>{
        self.csv_writers.iter().position(|r| r.index == index)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::packet::{Packet, PacketFieldValue};

    use super::FileHandler;

    #[test]
    fn test_write_packet(){

        let mut handler = FileHandler::default();
        let packet = Packet{
            structure_id: 0,
            field_data: vec![
                PacketFieldValue::UnsignedShort(0),
                PacketFieldValue::UnsignedShort(1),
                PacketFieldValue::UnsignedShort(0),
                PacketFieldValue::UnsignedShort(1),
            ],
            field_meta_data: vec![],
        };
        assert_eq!(handler.write_packet(packet).is_err(), false);
    }
    #[test]
    fn test_write_two_different_packets(){
        let mut handler = FileHandler::default();
        let mut packet = Packet{
            structure_id: 0,
            field_data: vec![
                PacketFieldValue::UnsignedShort(0),
                PacketFieldValue::UnsignedShort(1),
                PacketFieldValue::UnsignedShort(0),
                PacketFieldValue::UnsignedShort(1),
            ],
            field_meta_data: vec![],
        };
        assert_eq!(handler.write_packet(packet).is_err(), false);
        packet = Packet{
            structure_id: 1,
            field_data: vec![
                PacketFieldValue::UnsignedShort(3),
                PacketFieldValue::UnsignedShort(4),
                PacketFieldValue::UnsignedShort(5),
            ],
            field_meta_data: vec![],
        };
        assert_eq!(handler.write_packet(packet).is_err(), false);
    }
    #[test]
    fn test_can_adapt_to_write_error(){
        let mut handler = FileHandler::default();
        let mut packet = Packet{
            structure_id: 0,
            field_data: vec![
                PacketFieldValue::UnsignedShort(0),
                PacketFieldValue::UnsignedShort(1),
                PacketFieldValue::UnsignedShort(0),
                PacketFieldValue::UnsignedShort(1),
            ],
            field_meta_data: vec![],
        };
        assert_eq!(handler.write_packet(packet).is_err(), false);
        packet = Packet{
            structure_id: 0,
            field_data: vec![
                PacketFieldValue::UnsignedShort(3),
                PacketFieldValue::UnsignedShort(4),
                PacketFieldValue::UnsignedShort(5),
            ],
            field_meta_data: vec![],
        };
        assert_eq!(handler.write_packet(packet).is_err(), true);
        packet = Packet{
            structure_id: 0,
            field_data: vec![
                PacketFieldValue::UnsignedShort(3),
                PacketFieldValue::UnsignedShort(4),
                PacketFieldValue::UnsignedShort(5),
            ],
            field_meta_data: vec![],
        };
        assert_eq!(handler.write_packet(packet).is_err(), false);
    }
    #[test]
    fn test_write_bytes(){
        let mut handler = FileHandler::default();
        let data = vec![0,0xFF,0,0xFF];
        assert_eq!(handler.write_bytes(data).is_err(), false);
    }
    
}
