use std::{collections::HashMap, io};

use serde::{Deserialize, Serialize};

use crate::{data_id::DataId, reader::binary_reader::BinaryReader, types::string_id::StringId};

use super::string_info_data::StringInfoData;

#[derive(Serialize, Deserialize, Debug)]
pub struct StringInfo {
    pub string_id: StringId,
    pub table_id: DataId,
    pub variables: HashMap<u32, StringInfoData>,
    pub literal_value: String,
}
impl StringInfo {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        Ok(Self {
            string_id: todo!(),
            table_id: todo!(),
            variables: todo!(),
            literal_value: todo!(),
        })
    }
}
