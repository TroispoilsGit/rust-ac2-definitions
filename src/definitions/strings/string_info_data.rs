use std::io;

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::{reader::binary_reader::BinaryReader, types::double_precision::DoublePrecision};

use super::string_info::StringInfo;

#[derive(Serialize, Deserialize, Debug, FromPrimitive)]
pub enum DataType {
    Undef = 0,

    Int = 2,            // SID_Type_Int
    FormattedInt = 3,   // SID_Type_FormattedInt
    Float = 4,          // SID_Type_Float
    FormattedFloat = 5, // SID_Type_FormattedFloat
    UInt = 6,           // SID_Type_UInt
    FormattedUInt = 7,  // SID_Type_FormattedUInt

    StringInfo = 9,      // SID_Type_StringInfo
    LInt = 10,           // SID_Type_LInt
    FormattedLInt = 11,  // SID_Type_FormattedLInt
    ULInt = 12,          // SID_Type_ULInt
    FormattedULInt = 13, // SID_Type_FormattedULInt
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataValue {
    Int(i32),
    Double(DoublePrecision),
    Ushort(u16),
    Uint(u32),
    Long(i64),
    Ulong(u64),
    StringInfo(StringInfo),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StringInfoData {
    pub types: DataType,
    pub value: Option<DataValue>,
}
impl StringInfoData {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let types = data.read_enum16::<DataType>()?;
        data.align(4);

        let value = match types {
            DataType::Int => Some(DataValue::Int(data.read_i32()?)),
            DataType::Float => Some(DataValue::Double(DoublePrecision::new(data)?)),
            DataType::UInt => Some(DataValue::Uint(data.read_u32()?)),
            DataType::StringInfo => Some(DataValue::StringInfo(StringInfo::new(data)?)),
            DataType::LInt => Some(DataValue::Long(data.read_i64()?)),
            DataType::ULInt => Some(DataValue::Ulong(data.read_u64()?)),
            _ => None,
        };
        Ok(Self { types, value })
    }
}
