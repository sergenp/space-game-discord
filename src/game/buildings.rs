use std::collections::HashMap;

use super::{
    military::MilitaryCreationTypes,
    resource::{Resource, ResourceType},
    tickable::{TickResult, Tickable},
};

pub enum Building {
    MilitaryBuilding(MilitaryBuilding),
    ResourceBuilding(ResourceBuilding),
}

impl Tickable for Building {
    fn tick(&mut self) -> TickResult {
        match self {
            Self::MilitaryBuilding(mil) => mil.tick(),
            Self::ResourceBuilding(res) => res.tick(),
        }
    }
}

impl Building {
    pub fn get_building_data(&self) -> &BuildingData {
        match self {
            Self::MilitaryBuilding(mil) => &mil.building,
            Self::ResourceBuilding(res) => &res.building,
        }
    }
}
#[derive(Clone)]
pub struct BuildingData {
    pub name: String,
    pub resource_cost: HashMap<ResourceType, Resource>,
    pub level: u32,
}

#[derive(Clone)]
pub struct MilitaryBuilding {
    pub building: BuildingData,
    pub create_type: MilitaryCreationTypes,
    pub military_cost_per_tick: HashMap<ResourceType, Resource>,
    pub progress_required_to_create_military: u32,
    pub current_progress: u32,
    pub progress_increase_per_tick: u32,
}

impl MilitaryBuilding {
    pub fn new(
        name: String,
        resource_cost: HashMap<ResourceType, Resource>,
        create_type: MilitaryCreationTypes,
        military_cost_per_tick: HashMap<ResourceType, Resource>,
        progress_required_to_create_military: u32,
        progress_increase_per_tick: u32,
    ) -> Self {
        Self {
            create_type,
            building: BuildingData {
                name,
                resource_cost,
                level: 1,
            },
            military_cost_per_tick,
            progress_required_to_create_military,
            current_progress: 0,
            progress_increase_per_tick,
        }
    }
}
#[derive(Clone)]
pub struct ResourceBuilding {
    pub building: BuildingData,
    pub create_type: HashMap<ResourceType, Resource>,
}

impl Tickable for MilitaryBuilding {
    fn tick(&mut self) -> TickResult {
        self.current_progress += self.progress_increase_per_tick;
        if self.current_progress >= self.progress_required_to_create_military {
            self.current_progress = 0;
            TickResult::MilitaryBuildResult(self.create_type.clone())
        } else {
            TickResult::ResourceResult(self.military_cost_per_tick.clone())
        }
    }
}

impl Tickable for ResourceBuilding {
    fn tick(&mut self) -> TickResult {
        TickResult::ResourceResult(self.create_type.clone())
    }
}
