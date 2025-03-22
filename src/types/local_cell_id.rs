use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LocalCellId {
    pub indoor_id: u8,
    pub outdoor_id: u8,
}

impl LocalCellId {
    pub fn new(id: u16) -> Self {
        Self {
            indoor_id: ((id >> 8) & 0xFF) as u8,
            outdoor_id: (id & 0xFF) as u8,
        }
    }
}
