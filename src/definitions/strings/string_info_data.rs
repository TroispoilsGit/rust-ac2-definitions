use serde::{Deserialize, Serialize};

use super::string_info::StringInfo;

#[derive(Serialize, Deserialize, Debug)]
enum DataType {
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
enum DataValue {
    Int(i32),
    Double(f64),
    Ushort(u16),
    Uint(u32),
    Long(i64),
    Ulong(u64),
    StringInfo(StringInfo),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StringInfoData {
    pub types: DataType,
    pub value: DataValue,
}
