#[derive(Default)]
#[allow(dead_code)]
//the ftdi is an old software project from january 2024, allowing wings to interface with spi and i2c, 
//this ended up being abandoned because the ftdi4222 did not work as expected( would send a blank byte at the start of evergy transmission)
//if something like this is ever attempted again, just use a arduino or teensy or something instead, no ftdi.
pub struct FtdiSpiDriver {
    communciation_protocol: u8,
    is_syncronous: bool
}