use std::io;

use serde::{Deserialize, Serialize};

use crate::reader::binary_reader::BinaryReader;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct LinkDesc {
    pub id: i64,
    pub types: u32,
}

impl LinkDesc {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        Ok(Self {
            id: data.read_i64()?,
            types: data.read_u32()?,
        })
    }
}
