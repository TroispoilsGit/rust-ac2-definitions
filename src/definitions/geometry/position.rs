use std::io;

use serde::{Deserialize, Serialize};

use crate::reader::binary_reader::BinaryReader;

use super::frame::Frame;

#[derive(Serialize, Deserialize, Debug)]
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
