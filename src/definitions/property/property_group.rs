use super::base_property::BaseProperty;

pub struct PropertyGroup {
    pub properties: Vec<BaseProperty>,
}

impl PropertyGroup {
    pub fn new(properties: Vec<BaseProperty>) -> Self {
        Self { properties }
    }
}
