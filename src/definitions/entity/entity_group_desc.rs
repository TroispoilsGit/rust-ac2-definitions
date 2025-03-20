use std::io;

use crate::reader::binary_reader::BinaryReader;

use super::{entity_desc::EntityDesc, entity_link_desc::EntityLinkDesc};

pub struct EntityGroupDesc {
    pub entities: Vec<EntityDesc>,
    pub links: Vec<EntityLinkDesc>,
}

impl EntityGroupDesc {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let entities = data.read_list(|d| EntityDesc::new(d), 4)?;
        let links = data.read_list(|d| EntityLinkDesc::new(d), 4)?;
        Ok(EntityGroupDesc { entities, links })
    }
}
