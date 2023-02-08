use std::collections::HashSet;

use super::{
    military::MilitaryCreation,
    resource::{self, Resource, ResourceType},
    tickable::{TickResult, Tickable},
};

pub trait Building: Tickable {
    fn get_building_data(&self) -> &BuildingData;
}

pub struct BuildingData {
    pub name: String,
    pub resource_cost: ResourceType,
    pub level: u32,
}

impl BuildingData {
    fn new() {
        resource_cost = ResourceType::Credits(0);
    }

    fn add_1(&mut self, resource: ResourceType) {
        // self.resource_cost += resource;
        let res_cost = match self.resource_cost {
            ResourceType::Minerals(x) => x,
            ResourceType::Credits(x) => x,
            ResourceType::Food(x) => x,
        };
        self.resource_cost = match resource {
            ResourceType::Minerals(x) => ResourceType::Minerals(x + res_cost),
            ResourceType::Credits(x) => ResourceType::Credits(x + res_cost),
            ResourceType::Food(x) => ResourceType::Food(x + res_cost),
        }
    }
}

pub struct MilitaryBuilding {
    pub building: BuildingData,
    pub create_type: MilitaryCreation,
    pub military_resource_cost: HashSet<ResourceType>,
    pub next_creation_progress: u32,
    pub current_progress: u32,
}

impl MilitaryBuilding {
    pub fn new(
        create_type: MilitaryCreation,
        name: String,
        resource_cost: Resource,
        military_resource_cost: HashSet<ResourceType>,
        next_creation_progress: u32,
    ) -> Self {
        Self {
            create_type,
            building: BuildingData {
                name,
                resource_cost,
                level: 1,
            },
            military_resource_cost,
            next_creation_progress,
            current_progress: 0,
        }
    }
}

pub struct ResourceBuilding {
    pub building: BuildingData,
    pub create_type: Vec<Resource>,
}

impl Building for ResourceBuilding {
    fn get_building_data(&self) -> &BuildingData {
        &self.building
    }
}

impl Building for MilitaryBuilding {
    fn get_building_data(&self) -> &BuildingData {
        &self.building
    }
}

impl Tickable for MilitaryBuilding {
    fn tick(&mut self) -> TickResult {
        todo!();
    }
}

impl Tickable for ResourceBuilding {
    fn tick(&mut self) -> TickResult {
        TickResult::ResourceResult(self.create_type.clone())
    }
}
