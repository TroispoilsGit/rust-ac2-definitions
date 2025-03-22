use core::cmp::Ordering;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct DataId {
    id: u32,
}

impl DataId {
    pub fn new(id: u32) -> Self {
        DataId { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Clone for DataId {
    fn clone(&self) -> Self {
        DataId { id: self.id }
    }
}

impl Ord for DataId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for DataId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Implémentation personnalisée de Serialize
impl Serialize for DataId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{:08X}", self.id))
    }
}

// Implémentation personnalisée de Deserialize
impl<'de> Deserialize<'de> for DataId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some(hex) = s.strip_prefix("0x") {
            u32::from_str_radix(hex, 16)
                .map(DataId::new)
                .map_err(serde::de::Error::custom)
        } else {
            Err(serde::de::Error::custom("Invalid hex format"))
        }
    }
}
