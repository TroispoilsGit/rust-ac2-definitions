use std::io;

use crate::{
    entity::entity_group_desc::EntityGroupDesc, property::property_collection::PropertyCollection,
    reader::binary_reader::BinaryReader, types::data_id::DataId,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct CLandBlockInfo {
    pub data_did: DataId,
    pub version: u32,
    pub num_cells: u32,
    pub light_info_did: DataId,
    pub entities: Option<EntityGroupDesc>,
    pub properties: Option<PropertyCollection>,
    pub outside_stab_list: Vec<u32>,
}

impl CLandBlockInfo {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let has_version = data.read_bool()?;
        let data_did = data.read_dataid()?;
        let version = if has_version { data.read_u32()? } else { 0 };

        let num_cells = data.read_u32()?;
        let light_info_did = data.read_dataid()?;
        let entities = match EntityGroupDesc::new(data) {
            Ok(entity) => Some(entity),
            Err(_) => None,
        };
        let properties = match PropertyCollection::new(data) {
            Ok(property) => Some(property),
            Err(_) => None,
        };
        let outside_stab_list = data.read_list(|d| d.read_u32(), 4)?;

        Ok(Self {
            data_did,
            version,
            entities,
            properties,
            light_info_did,
            outside_stab_list,
            num_cells,
        })
    }
}
