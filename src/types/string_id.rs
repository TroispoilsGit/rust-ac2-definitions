use serde::{Deserialize, Serialize};

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
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
