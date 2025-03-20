use std::io;

use crate::reader::binary_reader::BinaryReader;

use super::base_property::BaseProperty;

pub struct PropertyGroup {
    pub properties: Vec<BaseProperty>,
}

impl PropertyGroup {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let properties = data.read_list(|d| BaseProperty::new(d), 4)?;
        Ok(Self { properties })
    }
}
