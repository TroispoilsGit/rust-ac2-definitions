use std::io;

use serde::{Deserialize, Serialize};

use crate::{
    property::property_collection::PropertyCollection,
    reader::binary_reader::BinaryReader,
    strings::encoding::{Encoding, EncodingType},
};

use super::link_desc::LinkDesc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct EntityLinkDesc {
    pub name: String,
    pub input: LinkDesc,
    pub output: LinkDesc,
    pub properties: PropertyCollection,
}
impl EntityLinkDesc {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        Ok(Self {
            name: data.read_string(Encoding::new(EncodingType::Ascii))?,
            input: LinkDesc::new(data)?,
            output: LinkDesc::new(data)?,
            properties: PropertyCollection::new(data)?,
        })
    }
}
