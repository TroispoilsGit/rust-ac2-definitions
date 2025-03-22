use std::io;

use serde::{Deserialize, Serialize};

use crate::{data_id::DataId, reader::binary_reader::BinaryReader};

use super::enums::texture_type::TextureType;

#[derive(Serialize, Deserialize, Debug)]
pub struct RenderTexture {
    pub did: DataId,
    pub texture_type: TextureType,
    pub level_surface_dids: Vec<DataId>,
}

impl RenderTexture {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let did = data.read_dataid()?;
        let texture_type = data.read_enum::<TextureType>()?;
        let level_surface_dids = data.read_list(|d| d.read_dataid(), 4)?;

        Ok(Self {
            did,
            texture_type,
            level_surface_dids,
        })
    }
}
