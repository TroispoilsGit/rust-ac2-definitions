use std::{collections::HashMap, io};

use serde::{Deserialize, Serialize};

use crate::{data_id::DataId, reader::binary_reader::BinaryReader, types::string_id::StringId};

use super::string_table_string::StringTableString;

#[derive(Serialize, Deserialize, Debug)]
pub struct StringTable {
    pub did: DataId,
    pub current_state_id: u32,
    pub base_val: u16,
    pub num_decimal_digits: u16,
    pub leading_zero: bool,
    pub strings: HashMap<StringId, StringTableString>,
}

impl StringTable {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let did = data.read_dataid()?;
        let current_state_id = data.read_u32()?;
        let base_val = data.read_u16()?;
        let num_decimal_digits = data.read_u16()?;
        let leading_zero = data.read_bool()?;
        let strings = data.read_dictionary(|d| d.read_stringid(), |d| StringTableString::new(d))?;

        Ok(Self {
            did,
            current_state_id,
            base_val,
            num_decimal_digits,
            leading_zero,
            strings,
        })
    }
}
