use std::{collections::HashMap, io};

use crate::{data_id::DataId, reader::binary_reader::BinaryReader};

#[derive(Debug, Clone)]
pub struct EnumMapper {
    pub did: DataId,
    pub base_enum_mapper_did: DataId,
    pub id_to_string: HashMap<u32, String>,
}

impl EnumMapper {
    pub fn new(cursor: &mut BinaryReader) -> io::Result<Self> {
        let did = cursor.read_dataid()?;
        let base_enum_mapper_did = cursor.read_dataid()?;
        let id_to_string = cursor.read_string_map()?;
        Ok(EnumMapper {
            did,
            base_enum_mapper_did,
            id_to_string,
        })
    }
}
