use std::io;

use crate::reader::binary_reader::BinaryReader;

use super::enums::{
    property_group_name::PropertyGroupName, property_type::PropertyType,
    weenie_report_type::WeenieReportType,
};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct BasePropertyDesc {
    pub types: PropertyType,
    pub group: PropertyGroupName,
    pub prop_data: u32,
    pub report_to_weenie: WeenieReportType,
    pub required: bool,
}

impl BasePropertyDesc {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let types = data.read_enum::<PropertyType>()?;
        let group = data.read_enum::<PropertyGroupName>()?;
        let prop_data = data.read_u32()?;
        let report_to_weenie = data.read_enum::<WeenieReportType>()?;
        let required = data.read_bool()?;

        Ok(BasePropertyDesc {
            types,
            group,
            prop_data,
            report_to_weenie,
            required,
        })
    }
}
