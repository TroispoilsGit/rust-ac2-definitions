use crate::reader::binary_reader::BinaryReader;

#[derive(Debug)]
pub struct DatHeader {
    pub ac_version_string: [u8; 256],
    pub disk_transact_info: DiskTransactInfo,
    pub disk_file_info: DiskFileInfo,
}

impl DatHeader {
    pub fn new(cursor: &mut BinaryReader) -> Result<Self, std::io::Error> {
        let mut ac_version_string = [0u8; 256];
        cursor.read_exact(&mut ac_version_string)?;

        let disk_transact_info = DiskTransactInfo::new(cursor)?;
        let disk_file_info = DiskFileInfo::new(cursor)?;

        Ok(DatHeader {
            ac_version_string,
            disk_transact_info,
            disk_file_info,
        })
    }
}

#[derive(Debug)]
pub struct DiskTransactInfo {
    pub dat_type: u32,
    pub ul: [u32; 10],
}

impl DiskTransactInfo {
    pub fn new(cursor: &mut BinaryReader) -> Result<Self, std::io::Error> {
        let dat_type = cursor.read_u32()?;
        let mut ul = [0u32; 10];
        for i in 0..10 {
            ul[i] = cursor.read_u32()?;
        }

        Ok(DiskTransactInfo { dat_type, ul })
    }
}

#[derive(Debug)]
pub struct DatIdStamp {
    pub data_1: u32,
    pub data_2: u16,
    pub data_3: u16,
    pub data_4: [u8; 8],
    pub minor_version: u32,
}

impl DatIdStamp {
    pub fn new(cursor: &mut BinaryReader) -> Result<Self, std::io::Error> {
        let data_1 = cursor.read_u32()?;
        let data_2 = cursor.read_u16()?;
        let data_3 = cursor.read_u16()?;
        let mut data_4 = [0u8; 8];
        cursor.read_exact(&mut data_4)?;
        let minor_version = cursor.read_u32()?;

        Ok(DatIdStamp {
            data_1,
            data_2,
            data_3,
            data_4,
            minor_version,
        })
    }
}

#[derive(Debug)]
pub struct DiskFileInfo {
    pub magic: u32,
    pub block_size: u32,
    pub file_size: u32,
    pub data_set_lm: u32,
    pub data_sub_set_lm: u32,
    pub first_free_block: u32,
    pub final_free_block: u32,
    pub num_free_blocks: u32,
    pub tree_root_offset: u32,
    pub young_lru_lm: u32,
    pub old_lru_lm: u32,
    pub use_lru_fm: bool,
    pub master_map_id: u32,
    pub eng_pack_version: u32,
    pub game_pack_version: u32,
    pub id_version: DatIdStamp,
}

impl DiskFileInfo {
    pub fn new(cursor: &mut BinaryReader) -> Result<Self, std::io::Error> {
        let magic = cursor.read_u32()?;
        let block_size = cursor.read_u32()?;
        let file_size = cursor.read_u32()?;
        let data_set_lm = cursor.read_u32()?;
        let data_sub_set_lm = cursor.read_u32()?;
        let first_free_block = cursor.read_u32()?;
        let final_free_block = cursor.read_u32()?;
        let num_free_blocks = cursor.read_u32()?;
        let tree_root_offset = cursor.read_u32()?;
        let young_lru_lm = cursor.read_u32()?;
        let old_lru_lm = cursor.read_u32()?;
        let use_lru_fm = cursor.read_bool()?;
        let master_map_id = cursor.read_u32()?;
        let eng_pack_version = cursor.read_u32()?;
        let game_pack_version = cursor.read_u32()?;
        let id_version = DatIdStamp::new(cursor)?;

        Ok(DiskFileInfo {
            magic,
            block_size,
            file_size,
            data_set_lm,
            data_sub_set_lm,
            first_free_block,
            final_free_block,
            num_free_blocks,
            tree_root_offset,
            young_lru_lm,
            old_lru_lm,
            use_lru_fm,
            master_map_id,
            eng_pack_version,
            game_pack_version,
            id_version,
        })
    }
}
