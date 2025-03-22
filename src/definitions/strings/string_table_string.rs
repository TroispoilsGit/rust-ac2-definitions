use std::io;

use serde::{Deserialize, Serialize};

use crate::reader::binary_reader::BinaryReader;

use super::encoding::{Encoding, EncodingType};

#[derive(Serialize, Deserialize, Debug)]
pub struct StringTableString {
    pub table: u32,
    pub strings: Vec<String>,
    pub variables: Vec<u32>,
    pub var_names: Vec<String>,
}

impl StringTableString {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let table = data.read_u32()?;
        let has_var_names_and_num_strings = data.read_u16()?;
        let has_var_names = (has_var_names_and_num_strings & 0x8000) != 0;
        let num_strings = has_var_names_and_num_strings & 0x7FFF;
        let mut strings = Vec::new();
        for _ in 0..num_strings {
            let string_not_empty = data.read_u16()? != 0;
            data.align(4);
            if string_not_empty {
                strings.push(data.read_string(Encoding::new(EncodingType::Unicode))?);
            } else {
                strings.push(String::new());
            }
        }
        data.align(4);
        let variables = data.read_list(|d| d.read_u32(), 2)?;
        data.align(4);
        let var_names = if has_var_names {
            let val = data.read_list(|d| d.read_string(Encoding::new(EncodingType::Unicode)), 4)?;
            val
        } else {
            Vec::new()
        };

        Ok(Self {
            table,
            strings,
            variables,
            var_names,
        })
    }
}
