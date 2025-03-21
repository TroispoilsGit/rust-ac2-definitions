use std::io;

use serde::{Deserialize, Serialize};

use crate::{
    geometry::{position::Position, waveform::Waveform},
    reader::binary_reader::BinaryReader,
    strings::{
        encoding::{Encoding, EncodingType},
        string_info::StringInfo,
    },
};

use super::{
    enums::{
        property_group_name::PropertyGroupName, property_name::PropertyName,
        property_type::PropertyType, property_value::PropertyValue,
    },
    master_property::MasterProperty,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct BaseProperty {
    pub name: PropertyName,
    pub types: PropertyType,
    pub group: PropertyGroupName,
    pub value: Option<PropertyValue>,
}

impl BaseProperty {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let name = data.read_enum::<PropertyName>()?;
        let master_property = MasterProperty::instance().expect("issue MasterProperty");
        let base_property_desc = master_property
            .properties
            .get(&name)
            .expect("name don't existe into MasterProperty.");
        let types = base_property_desc.types;
        let group = base_property_desc.group;

        let value = match types {
            PropertyType::Bool => Some(PropertyValue::Bool(data.read_bool()?)),
            PropertyType::Integer => Some(PropertyValue::Integer(data.read_i32()?)),
            PropertyType::Float => Some(PropertyValue::Float(data.read_f32()?)),
            PropertyType::Vector => Some(PropertyValue::Vector(data.read_vector3()?)),
            PropertyType::Color => Some(PropertyValue::Color(data.read_color()?)),
            PropertyType::String => Some(PropertyValue::String(
                data.read_string(Encoding::new(EncodingType::Utf8))?,
            )),
            PropertyType::Enum => Some(PropertyValue::Enum(data.read_u32()?)),
            PropertyType::DataFile => Some(PropertyValue::DataFile(data.read_dataid()?)),
            PropertyType::Waveform => Some(PropertyValue::Waveform(Waveform::new(data)?)),
            PropertyType::StringInfo => Some(PropertyValue::StringInfo(
                StringInfo::new(data).expect("Issue with stringinfo"),
            )),
            PropertyType::PackageID => Some(PropertyValue::PackageID(data.read_u32()?)),
            PropertyType::LongInteger => Some(PropertyValue::LongInteger(data.read_i64()?)),
            PropertyType::Position => Some(PropertyValue::Position(Position::new(data)?)),
            _ => None,
        };

        Ok(BaseProperty {
            name,
            types,
            group,
            value,
        })
    }
}
