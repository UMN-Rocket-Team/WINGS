//specific communications device for reading files

use std::{fs::File, io::Read};

use anyhow::bail;

use crate::communication_manager::{CommsIF, DeviceName};

#[derive(Default)]
pub struct ByteReadDriver {
    file: Option<File>,
    id: usize,
}
impl CommsIF for ByteReadDriver{
    fn init_device(&mut self, file_name: &str, _baud: u32)  -> anyhow::Result<()> {
        match File::open(file_name){
            Ok(new_file) => {
                self.file = Some(new_file); 
                Ok(())
            },
            Err(err) => bail!(err),
        }
    }

    //This file should never have bytes written to it by wings. look at file_handling.rs to see how we write data
    fn write_port(&mut self, packet: &[u8])  -> anyhow::Result<()> {
        let _ = packet;
        Ok(())
    }

    fn read_port(&mut self, write_buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        if self.is_init(){
            let mut buffer = [0; 4096];
            match self.file.as_mut().unwrap().read(&mut buffer) {
                Ok(_) => {
                    if buffer == [0; 4096] {
                        return Ok(());
                    }
                    write_buffer.extend_from_slice(&buffer); 
                    Ok(())
                },
                Err(err) =>  bail!(err),
            }
        }
        else{
            bail!("reading from uninitialized driver");
        }
    }

    //Picking files to read is done by the frontend. We don't need to worry about scanning for files that the user might want
    fn get_new_available_ports(&mut self) -> std::option::Option<Vec<DeviceName>> {
        return None;
    }

    fn is_init(&mut self) -> bool {
        self.file.is_some()
    }
    fn set_id(&mut self, id: usize){
        self.id = id;
    }
    fn get_id(&self) -> usize {
        return self.id;
    }

    fn get_type(&self) -> String {
        return "ByteFile".to_owned();
    }
}