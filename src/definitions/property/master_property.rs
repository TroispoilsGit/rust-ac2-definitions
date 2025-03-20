use std::collections::HashMap;
use std::io::{self};
use std::sync::{Arc, OnceLock};

use crate::DatReader;
use crate::data_id::DataId;
use crate::enum_map::enum_mapper::EnumMapper;
use crate::reader::binary_reader::BinaryReader;

use super::base_property_desc::BasePropertyDesc;
use super::enums::property_name::PropertyName;

static INSTANCE: OnceLock<Arc<MasterProperty>> = OnceLock::new();
static MASTER_PROPERTY_DID: u32 = 0x34000000;

#[derive(Debug, Clone)]
pub struct MasterProperty {
    pub did: DataId,                                         // m_DID
    pub enum_mapper: EnumMapper,                             // m_emapper
    pub properties: HashMap<PropertyName, BasePropertyDesc>, // m_properties
}

impl MasterProperty {
    // Charge les propriétés maîtres depuis le fichier
    pub fn load_master_properties(dat_reader: &mut DatReader) {
        INSTANCE.get_or_init(|| {
            let mut data = dat_reader.get_binary_file_u32(MASTER_PROPERTY_DID).unwrap();
            Arc::new(MasterProperty::new(&mut data).unwrap())
        });
    }

    // Constructeur à partir d'un lecteur
    pub fn new(data: &mut BinaryReader) -> io::Result<Self> {
        let did = data.read_dataid()?;
        let enum_mapper = EnumMapper::new(data)?;
        let mut properties = HashMap::new();

        let num_elements = data.read_u16().unwrap();
        let _table_size = data.read_u16(); // Ignoré pour l'instant, mais tu peux l'utiliser si nécessaire

        properties.reserve(num_elements as usize); // Équivalent de `EnsureCapacity` en C#

        for _ in 0..num_elements {
            properties.insert(
                data.read_enum::<PropertyName>()?,
                BasePropertyDesc::new(data)?,
            );
        }

        Ok(MasterProperty {
            did,
            enum_mapper,
            properties,
        })
    }

    // Récupérer l'instance unique
    pub fn instance() -> Option<Arc<MasterProperty>> {
        INSTANCE.get().cloned()
    }
}
