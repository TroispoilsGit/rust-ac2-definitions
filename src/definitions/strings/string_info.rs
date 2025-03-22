use std::{collections::HashMap, io};

use serde::{Deserialize, Serialize};

use crate::{data_id::DataId, reader::binary_reader::BinaryReader, types::string_id::StringId};

use super::{
    encoding::{Encoding, EncodingType},
    string_info_data::StringInfoData,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct StringInfo {
    pub string_id: StringId,
    pub table_id: DataId,
    pub variables: HashMap<u32, StringInfoData>,
    pub literal_value: String,
}
impl StringInfo {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let string_id = data.read_stringid()?;
        let table_id = data.read_dataid()?;
        let num_variables = data.read_u16()?;
        let literal_value = if data.read_u16()? != 0 {
            data.read_string(Encoding::new(EncodingType::Unicode))?
        } else {
            String::new()
        };

        let mut variables = HashMap::new();
        for _ in 0..num_variables {
            let key = data.read_u32()?;
            let value = StringInfoData::new(data)?;
            variables.insert(key, value);
        }

        Ok(Self {
            string_id,
            table_id,
            variables,
            literal_value,
        })
    }
}
