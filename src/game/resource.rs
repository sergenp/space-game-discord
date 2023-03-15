use std::ops::AddAssign;

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

impl AddAssign for Resource {
    fn add_assign(&mut self, rhs: Self) {
        match rhs.resource_type == self.resource_type {
            true => {
                *self = Self {
                    resource_type: rhs.resource_type,
                    amount: self.amount + rhs.amount,
                }
            }
            false => panic!("Resource type mismatch"),
        }
    }
}
