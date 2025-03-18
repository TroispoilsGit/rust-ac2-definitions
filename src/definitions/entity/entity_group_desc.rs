pub struct EntityGroupDesc {
    pub entities: Vec<EntityDesc>,
    pub links: Vec<EntityLinkDesc>,
}

impl EntityGroupDesc {
    pub fn new() -> Self {
        EntityGroupDesc {
            entities: Vec::new(),
            links: Vec::new(),
        }
    }
}
