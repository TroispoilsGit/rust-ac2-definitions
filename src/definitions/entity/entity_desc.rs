use std::io;

use crate::{
    data_id::DataId,
    property::property_collection::PropertyCollection,
    reader::binary_reader::BinaryReader,
    strings::encoding::{Encoding, EncodingType},
    types::{matrix4x4::Matrix4x4, vector3::Vector3},
    utils::flags::Flags,
};

use super::enums::entity_type::EntityType;

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum PackFlag {
    NONE = 0,
    DATABASE = 1 << 0,   // DATABASE 0x00000001
    TYPE = 1 << 1,       // TYPE 0x00000002
    RUNTIMEID = 1 << 2,  // RUNTIMEID 0x00000004
    DATAID = 1 << 3,     // DATAID 0x00000008
    OFFSET = 1 << 4,     // OFFSET 0x00000010
    SCALE = 1 << 5,      // SCALE 0x00000020
    VERSION = 1 << 6,    // VERSION 0x00000040
    PROPERTIES = 1 << 7, // PROPERTIES 0x00000080
    WBNAME = 1 << 8,     // WBNAME 0x00000100
}

impl From<PackFlag> for u32 {
    fn from(flag: PackFlag) -> Self {
        flag as u32
    }
}

pub struct EntityDesc {
    pub pack_flags: u32,
    pub did: DataId,
    pub entity_type: EntityType,
    pub runtime_id: u64,
    pub data_id: DataId,
    pub package_type: u32, //TODO: to be define enum
    pub offset: Matrix4x4,
    pub scale: Vector3,
    pub properties: Option<PropertyCollection>,
    pub version: u32,
    pub wb_id: u64,
    pub wb_name: String,
    pub comments: String,
    pub inside: bool,
}
impl EntityDesc {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let pack_flags = data.read_u32()?;

        let did = if Flags::has_flag_enum(pack_flags, PackFlag::DATABASE) {
            let did = data.read_dataid()?;
            did
        } else {
            DataId::new(0)
        };

        let entity_type = if Flags::has_flag_enum(pack_flags, PackFlag::TYPE) {
            let entity_type = data.read_enum::<EntityType>()?;
            entity_type
        } else {
            EntityType::Invalid
        };

        let runtime_id = if Flags::has_flag_enum(pack_flags, PackFlag::RUNTIMEID) {
            let runtime_id = data.read_u64()?;
            runtime_id
        } else {
            0
        };

        let (data_id, package_type) = if Flags::has_flag_enum(pack_flags, PackFlag::DATAID) {
            let id = data.read_u32()?;
            let data_id = DataId::new(id);
            let package_type = id;
            (data_id, package_type)
        } else {
            (DataId::new(0), 0)
        };

        let offset = if Flags::has_flag_enum(pack_flags, PackFlag::OFFSET) {
            let offset = data.read_matrix4x4()?;
            offset
        } else {
            Matrix4x4::identity()
        };

        let scale = if Flags::has_flag_enum(pack_flags, PackFlag::SCALE) {
            let scale = data.read_vector3()?;
            scale
        } else {
            Vector3::new(1.0, 1.0, 1.0)
        };

        let properties: Option<PropertyCollection> =
            if Flags::has_flag_enum(pack_flags, PackFlag::PROPERTIES) {
                let properties = PropertyCollection::new(data)?;
                Some(properties)
            } else {
                None
            };

        let version = if Flags::has_flag_enum(pack_flags, PackFlag::VERSION) {
            let version = data.read_u32()?;
            version
        } else {
            0
        };

        let wb_name = if Flags::has_flag_enum(pack_flags, PackFlag::WBNAME) {
            let wb_name = data.read_string(Encoding::new(EncodingType::Ascii))?;
            wb_name
        } else {
            String::new()
        };

        Ok(Self {
            pack_flags,
            did,
            entity_type,
            runtime_id,
            data_id,
            package_type,
            offset,
            scale,
            properties,
            version,
            wb_id: 0,
            wb_name,
            comments: String::new(),
            inside: false,
        })
    }
}
