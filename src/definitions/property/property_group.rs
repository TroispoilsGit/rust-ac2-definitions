use std::io;

use serde::{Deserialize, Serialize};

use crate::reader::binary_reader::BinaryReader;

use super::{base_property::BaseProperty, enums::property_group_name::PropertyGroupName};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct PropertyGroup {
    pub name: PropertyGroupName,
    pub properties: Vec<BaseProperty>,
}

impl PropertyGroup {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let name = data.read_enum::<PropertyGroupName>()?;
        let properties = data.read_list(|d| BaseProperty::new(d), 4)?;
        Ok(Self { name, properties })
    }
}
