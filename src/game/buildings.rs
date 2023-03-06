use std::collections::HashMap;

use super::{
    military::MilitaryCreation,
    resource::{Resource, ResourceType},
    tickable::{TickResult, Tickable},
};

pub trait Building: Tickable {
    fn get_building_data(&self) -> &BuildingData;
}

pub struct BuildingData {
    pub name: String,
    pub resource_cost: HashMap<ResourceType, Resource>,
    pub level: u32,
}

pub struct MilitaryBuilding {
    pub building: BuildingData,
    pub create_type: MilitaryCreation,
    pub next_creation_progress: u32,
    pub current_progress: u32,
}

impl MilitaryBuilding {
    pub fn new(
        create_type: MilitaryCreation,
        name: String,
        resource_cost: HashMap<ResourceType, Resource>,
        next_creation_progress: u32,
    ) -> Self {
        Self {
            create_type,
            building: BuildingData {
                name,
                resource_cost,
                level: 1,
            },
            next_creation_progress,
            current_progress: 0,
        }
    }
}

pub struct ResourceBuilding {
    pub building: BuildingData,
    pub create_type: HashMap<ResourceType, Resource>,
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
