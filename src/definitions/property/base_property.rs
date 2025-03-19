use std::io;

use crate::{
    geometry::{position::Position, waveform::Waveform},
    reader::binary_reader::BinaryReader,
    strings::string_info::StringInfo,
};

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
        //TODO: Generate MasterProperty.
        let value = match property_desc.property_type {
            PropertyType::Bool => PropertyValue::Bool(data.read_bool()?),
            PropertyType::Integer => PropertyValue::Integer(data.read_i32()?),
            PropertyType::Float => PropertyValue::Float(data.read_f32()?),
            PropertyType::Vector => PropertyValue::Vector(data.read_vector3()?),
            PropertyType::Color => PropertyValue::Color(data.read_color()?),
            PropertyType::String => PropertyValue::String(data.read_string()?),
            PropertyType::Enum => PropertyValue::Enum(data.read_u32()?),
            PropertyType::DataFile => PropertyValue::DataFile(data.read_dataid()?),
            PropertyType::Waveform => PropertyValue::Waveform(Waveform::new(data)?),
            PropertyType::StringInfo => PropertyValue::StringInfo(StringInfo::new(data)?),
            PropertyType::PackageID => PropertyValue::PackageID(data.read_u32()?),
            PropertyType::LongInteger => PropertyValue::LongInteger(data.read_i64()?),
            PropertyType::Position => PropertyValue::Position(Position::new(data)?),
        };
    }
}
