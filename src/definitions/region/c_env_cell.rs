use std::io;

use crate::{
    data_id::DataId, entity::entity_group_desc::EntityGroupDesc, geometry::position::Position,
    property::property_collection::PropertyCollection, reader::binary_reader::BinaryReader,
    utils::flags::Flags,
};

use super::c_cell_portal::CCellPortal;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PackFlag {
    None = 0,
    SeenOutside = 1 << 0,   // ENVCELL_PACK_SEEN_OUTSIDE 0x00000001
    HasEntities = 1 << 1,   // ENVCELL_PACK_HAS_ENTITIES 0x00000002
    HasProperties = 1 << 2, // ENVCELL_PACK_HAS_PROPERTIES 0x00000004
    NeverCull = 1 << 3,     // ENVCELL_PACK_NEVER_CULL 0x00000008
    DrawSky = 1 << 4,       // ENVCELL_PACK_DRAW_SKY 0x00000010
}

impl From<PackFlag> for u32 {
    fn from(flag: PackFlag) -> Self {
        flag as u32
    }
}

#[derive(Debug)]
pub struct CEnvCell {
    pub pack_flags: u32,
    pub pos: Position,
    pub stab_list: Vec<u16>, //TODO: class LocalCellID
    pub environment_did: DataId,
    pub portals: Vec<CCellPortal>,
    pub entities: Option<EntityGroupDesc>,
    pub properties: Option<PropertyCollection>,
    pub shared_cells: Vec<u16>, //TODO: class LocalCellID
}

impl CEnvCell {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let pos = Position::new(data)?;
        let pack_flags = data.read_u32()?;
        let environment_did = data.read_dataid()?;
        let portals = data.read_list(|b| CCellPortal::new(b), 4)?;
        let stab_list = data.read_list(|b| b.read_u16(), 4)?;
        data.align(4);
        let shared_cells = data.read_list(|b| b.read_u16(), 4)?;
        data.align(4);

        let entities = if Flags::has_flag_enum(pack_flags, PackFlag::HasEntities) {
            Some(EntityGroupDesc::new(data)?)
        } else {
            None
        };
        let properties = if Flags::has_flag_enum(pack_flags, PackFlag::HasProperties) {
            Some(PropertyCollection::new(data)?)
        } else {
            None
        };

        Ok(Self {
            pack_flags,
            pos,
            stab_list,
            environment_did,
            portals,
            entities,
            properties,
            shared_cells,
        })
    }
}
