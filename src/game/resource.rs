use std::ops::AddAssign;

#[derive(Clone, Copy)]
pub struct Resource {
    pub resource_type_minerals: ResourceType,
    pub amount: i32,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum ResourceType {
    Minerals(i32),
    Credits(i32),
    Food(i32),
}

impl ResourceType {
    pub fn iterator() -> impl Iterator<Item = ResourceType> {
        [
            ResourceType::Minerals(0),
            ResourceType::Credits(0),
            ResourceType::Food(0),
        ]
        .iter()
        .copied()
    }
}

impl AddAssign for ResourceType {
    fn add_assign(&mut self, rhs: Self) {
        let res_cost = match self {
            ResourceType::Minerals(x) => *x,
            ResourceType::Credits(x) => *x,
            ResourceType::Food(x) => *x,
        };
        *self = match rhs {
            ResourceType::Minerals(x) => ResourceType::Minerals(x + res_cost),
            ResourceType::Credits(x) => ResourceType::Credits(x + res_cost),
            ResourceType::Food(x) => ResourceType::Food(x + res_cost),
        }
    }
}
