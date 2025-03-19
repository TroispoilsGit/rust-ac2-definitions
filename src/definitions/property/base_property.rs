use std::io;

use crate::reader::binary_reader::BinaryReader;

use super::enums::{
    property_group_name::PropertyGroupName, property_name::PropertyName,
    property_type::PropertyType, property_value::PropertyValue,
};

pub struct BaseProperty {
    pub name: PropertyName,
    pub types: PropertyType,
    pub group: PropertyGroupName,
    pub value: PropertyValue,
}

impl BaseProperty {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let name = data.read_enum::<PropertyName>()?;
        let types = data.read_enum::<PropertyType>()?;
        let group = data.read_enum::<PropertyGroupName>()?;
        //Generate MasterProperty.
        let value = 1; //TODO: To be need continue.
    }
}
