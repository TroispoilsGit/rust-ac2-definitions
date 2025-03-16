use chrono::{TimeZone, Utc};

use crate::extensions::binary_reader::BinaryReader;

#[derive(Debug, Copy, Clone)]
pub struct NodeEntry {
    pub did: u32,
    pub offset: u32,
    pub size: u32,
    pub date: u32,
}

impl NodeEntry {
    pub fn new(cursor: &mut BinaryReader) -> Result<Self, std::io::Error> {
        let did = cursor.read_u32()?;
        let offset = cursor.read_u32()?;
        let size = cursor.read_u32()?;
        let date = cursor.read_u32()?;

        Ok(NodeEntry {
            did,
            offset,
            size,
            date,
        })
    }

    pub fn to_string(&self) -> String {
        let date_time = match Utc.timestamp_opt(self.date as i64, 0).single() {
            Some(dt) => dt.to_string(),
            None => "Invalid timestamp".to_string(),
        };

        format!(
            "did: 0x{:08X} (pos: {:08X} - size: {}) [date: {}]",
            self.did, self.offset, self.size, date_time
        )
    }
}
