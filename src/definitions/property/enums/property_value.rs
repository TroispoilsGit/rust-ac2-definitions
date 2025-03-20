use serde::{Deserialize, Serialize};

use crate::{
    data_id::DataId,
    geometry::{position::Position, waveform::Waveform},
    strings::string_info::StringInfo,
    types::{rgba_color::RGBAColor, vector3::Vector3},
};

#[derive(Serialize, Deserialize, Debug)]
pub enum PropertyValue {
    Bool(bool),
    Integer(i32),
    Float(f32),
    Vector(Vector3),
    Color(RGBAColor),
    String(String),
    Enum(u32),
    DataFile(DataId),
    Waveform(Waveform),
    StringInfo(StringInfo),
    PackageID(u32), //TODO: Enum packagetype
    LongInteger(i64),
    Position(Position),
}
