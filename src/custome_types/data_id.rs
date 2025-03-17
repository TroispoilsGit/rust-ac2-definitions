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
