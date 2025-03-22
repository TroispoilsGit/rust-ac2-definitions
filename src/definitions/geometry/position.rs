use std::io;

use serde::{Deserialize, Serialize};

use crate::{reader::binary_reader::BinaryReader, types::cell_id::CellId};

use super::frame::Frame;

#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    pub cell: CellId,
    pub frame: Frame,
}

impl Position {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let cell = data.read_cellid()?;
        let frame = Frame::new(data)?;

        Ok(Self { cell, frame })
    }
}
