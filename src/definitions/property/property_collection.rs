use std::io;

use serde::{Deserialize, Serialize};

use crate::reader::binary_reader::BinaryReader;

use super::property_group::PropertyGroup;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct PropertyCollection {
    pub groups: Vec<PropertyGroup>,
}

impl PropertyCollection {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let groups = data.read_list(|d| PropertyGroup::new(d), 4)?;
        Ok(Self { groups })
    }
}
