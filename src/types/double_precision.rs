use std::io;

use serde::{Deserialize, Serialize};

use crate::reader::binary_reader::BinaryReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct DoublePrecision {
    pub double: f64,
    pub precision: u16,
}

impl DoublePrecision {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let double = data.read_f64()?;
        let precision = data.read_u16()?;
        data.align(4);

        Ok(Self { double, precision })
    }
}
