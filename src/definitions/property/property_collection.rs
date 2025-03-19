use super::property_group::PropertyGroup;

pub struct PropertyCollection {
    pub groups: Vec<PropertyGroup>,
}

impl PropertyCollection {
    pub fn new(groups: Vec<PropertyGroup>) -> Self {
        Self { groups }
    }
}
