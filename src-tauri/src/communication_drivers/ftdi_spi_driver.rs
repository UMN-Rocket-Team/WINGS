#[derive(Default)]
#[allow(dead_code)]
pub struct FtdiSpiDriver {
    communciation_protocol: u8,
    is_syncronous: bool
}