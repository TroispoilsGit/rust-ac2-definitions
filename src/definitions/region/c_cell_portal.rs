use std::io;

use crate::{
    reader::binary_reader::BinaryReader, types::local_cell_id::LocalCellId, utils::flags::Flags,
};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum PackFlag {
    None = 0,
    ExactMatch = 1 << 0,    // CELLPORTAL_PACK_EXACT_MATCH 0x00000001
    NoOtherCellId = 1 << 1, // CELLPORTAL_PACK_NO_OTHER_CELL_ID 0x00000002
    StabList = 1 << 2,      // CELLPORTAL_PACK_STAB_LIST 0x00000004
}

impl From<PackFlag> for u32 {
    fn from(flag: PackFlag) -> Self {
        flag as u32
    }
}

#[derive(Debug)]
pub struct CCellPortal {
    pub pack_flags: u16,
    pub portal_index: u16,
    pub other_cell_id: LocalCellId,
    pub other_portal_index: u16,
    pub stab_list: Vec<LocalCellId>,
}

impl CCellPortal {
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let pack_flags = data.read_u16()?;
        let portal_index = data.read_u16()?;
        let other_cell_id = data.read_local_cellid()?;
        let other_portal_index = data.read_u16()?;

        let stab_list = if Flags::has_flag_enum16(pack_flags, PackFlag::StabList) {
            let list = data.read_list(|b| b.read_local_cellid(), 2)?;
            data.align(4);
            list
        } else {
            Vec::new()
        };

        Ok(CCellPortal {
            pack_flags,
            portal_index,
            other_cell_id,
            other_portal_index,
            stab_list,
        })
    }
}
