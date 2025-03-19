use std::collections::HashMap;

use crate::{data_id::DataId, types::string_id::StringId};

use super::string_info_data::StringInfoData;

#[derive(Debug)]
pub struct StringInfo {
    pub string_id: StringId,
    pub table_id: DataId,
    pub variables: HashMap<u32, StringInfoData>,
    pub literal_value: String,
}
