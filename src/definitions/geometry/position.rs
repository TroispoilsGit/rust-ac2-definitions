use std::io;

use crate::reader::binary_reader::BinaryReader;

use super::frame::Frame;

#[derive(Debug)]
pub struct Position {
    pub cell: u32, //TODO: cellid
    pub frame: Frame,
}

impl Position {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let cell = data.read_u32()?;
        let frame = Frame::new(data)?;

        Ok(Self { cell, frame })
    }
}
