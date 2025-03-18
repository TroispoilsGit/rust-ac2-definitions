use serde::{Deserialize, Serialize};
use std::io::Error;

use crate::{reader::binary_reader::BinaryReader, types::data_id::DataId};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct CLandBlockData {
    pub data_did: DataId,
    pub land_block_info_did: DataId,
    pub path_map_did: DataId,
    pub heights: Vec<u8>,
    pub cell_infos: Vec<u32>,
}

impl CLandBlockData {
    pub fn new(data: &mut BinaryReader) -> Result<Self, Error> {
        let data_did = data.read_dataid()?;
        let land_block_info_did = data.read_dataid()?;
        let path_map_did = data.read_dataid()?;
        let heights = data.read_list::<u8>()?;
        let cell_infos = data.read_list::<u32>()?;

        Ok(CLandBlockData {
            data_did,
            land_block_info_did,
            path_map_did,
            heights,
            cell_infos,
        })
    }
}
