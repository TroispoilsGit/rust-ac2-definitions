use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct StringId {
    id: u32,
}

impl StringId {
    pub fn new(id: u32) -> Self {
        StringId { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Clone for StringId {
    fn clone(&self) -> Self {
        StringId { id: self.id }
    }
}

// Implémentation personnalisée de Serialize
impl Serialize for StringId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{:08X}", self.id))
    }
}

// Implémentation personnalisée de Deserialize
impl<'de> Deserialize<'de> for StringId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Some(hex) = s.strip_prefix("0x") {
            u32::from_str_radix(hex, 16)
                .map(StringId::new)
                .map_err(serde::de::Error::custom)
        } else {
            Err(serde::de::Error::custom("Invalid hex format"))
        }
    }
}
