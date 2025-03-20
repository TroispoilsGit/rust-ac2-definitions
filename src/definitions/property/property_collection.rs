use std::io;

use crate::reader::binary_reader::BinaryReader;

use super::property_group::PropertyGroup;

pub struct PropertyCollection {
    pub groups: Vec<PropertyGroup>,
}

impl PropertyCollection {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let groups = data.read_list(|d| PropertyGroup::new(d), 4)?;
        Ok(Self { groups })
    }
}
