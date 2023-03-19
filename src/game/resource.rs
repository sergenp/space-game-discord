#[derive(Debug, Clone, Copy)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub amount: i32,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum ResourceType {
    Minerals,
    Credits,
    Food,
}
