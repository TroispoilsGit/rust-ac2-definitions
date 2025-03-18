use crate::{
    data_id::DataId,
    types::{matrix_4x4::Matrix4x4, vector_3::Vector3},
};

use super::enums::entity_type::EntityType;

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

pub struct EntityDesc {
    pub pack_flags: PackFlag,
    pub did: DataId,
    pub entity_type: EntityType,
    pub instance_id: u64,
    pub data_id: DataId,
    pub package_type: u32, //TODO: to be define enum
    pub offset: Matrix4x4,
    pub scale: Vector3,
}
