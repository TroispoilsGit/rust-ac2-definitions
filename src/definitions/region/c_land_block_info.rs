use serde::{Deserialize, Serialize};
use std::io::Error;

use crate::{reader::binary_reader::BinaryReader, types::data_id::DataId};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct CLandBlockInfo {
    pub data_did: DataId,
    pub version: u32,
}

impl CLandBlockInfo {
    pub fn new(data: &mut BinaryReader) -> Result<Self, Error> {
        let data_did = data.read_dataid()?;
        let version = data.read_u32()?;

        Ok(CLandBlockInfo { data_did, version })
    }
}
