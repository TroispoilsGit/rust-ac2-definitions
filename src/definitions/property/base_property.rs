use std::io;

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

pub struct BaseProperty {
    pub name: PropertyName,
    pub types: PropertyType,
    pub group: PropertyGroupName,
    pub value: PropertyValue,
}

impl BaseProperty {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let name = data.read_enum::<PropertyName>()?;
        let master_property = MasterProperty::instance().expect("issue MasterProperty");
        let base_property_desc = master_property
            .properties
            .get(&name)
            .expect("{name} : don't existe into MasterProperty.");
        let types = base_property_desc.types;
        let group = base_property_desc.group;

        let value = match types {
            PropertyType::Bool => PropertyValue::Bool(data.read_bool()?),
            PropertyType::Integer => PropertyValue::Integer(data.read_i32()?),
            PropertyType::Float => PropertyValue::Float(data.read_f32()?),
            PropertyType::Vector => PropertyValue::Vector(data.read_vector3()?),
            PropertyType::Color => PropertyValue::Color(data.read_color()?),
            PropertyType::String => {
                PropertyValue::String(data.read_string(Encoding::new(EncodingType::Utf8))?)
            }
            PropertyType::Enum => PropertyValue::Enum(data.read_u32()?),
            PropertyType::DataFile => PropertyValue::DataFile(data.read_dataid()?),
            PropertyType::Waveform => PropertyValue::Waveform(Waveform::new(data)?),
            PropertyType::StringInfo => PropertyValue::StringInfo(StringInfo::new(data)?),
            PropertyType::PackageID => PropertyValue::PackageID(data.read_u32()?),
            PropertyType::LongInteger => PropertyValue::LongInteger(data.read_i64()?),
            PropertyType::Position => PropertyValue::Position(Position::new(data)?),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown property type",
                ));
            }
        };

        Ok(BaseProperty {
            name,
            types,
            group,
            value,
        })
    }
}
