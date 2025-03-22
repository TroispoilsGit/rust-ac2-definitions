use serde::{Deserialize, Serialize};

use super::local_cell_id::LocalCellId;

#[derive(Serialize, Deserialize, Debug)]
pub struct CellId {
    pub landblock_x: u8,
    pub landblock_y: u8,
    pub local_cell_id: LocalCellId,
}

impl CellId {
    pub fn new(id: u32) -> Self {
        Self {
            landblock_x: ((id >> 24) & 0xFF) as u8,
            landblock_y: ((id >> 16) & 0xFF) as u8,
            local_cell_id: LocalCellId::new((id & 0xFFFF) as u16),
        }
    }
}
