use super::buildings::Building;
use super::military::Fleet;
use super::military::MilitaryCreationTypes;
use super::position::Position;
use super::resource::Resource;
use super::resource::ResourceType;
use super::tickable::TickResult;
use super::tickable::Tickable;
use std::collections::HashMap;

pub struct Planet {
    pub name: String,
    pub buildings: Vec<Building>,
    pub military: HashMap<String, Fleet>,
    pub resources: HashMap<ResourceType, Resource>,
    pub position: Position,
}

impl Planet {
    pub fn add_resource(
        resource_map: &mut HashMap<ResourceType, Resource>,
        resource_to_add: HashMap<ResourceType, Resource>,
    ) {
        for resource in resource_to_add {
            let planet_resource = match resource_map.get_mut(&resource.0) {
                Some(planet_resource) => planet_resource,
                None => {
                    panic!("No resource have been found in the planet for building the building.")
                }
            };

            planet_resource.amount += resource.1.amount;
        }
    }

    pub fn create_fleet(fleet_map: &mut HashMap<String, Fleet>, name: String) {
        fleet_map.insert(name, Fleet { ships: vec![] });
    }

    pub fn add_military(fleet: &mut Fleet, military: MilitaryCreationTypes) {
        fleet.ships.push(military);
    }

    pub fn get_resource(&self, resource_type: ResourceType) -> Option<&Resource> {
        return self.resources.get(&resource_type);
    }

    pub fn build_building(&mut self, building: Building) -> Result<(), &str> {
        let building_data = building.get_building_data();

        for resource in &building_data.resource_cost {
            let planet_resource = match self.resources.get(&resource.0) {
                Some(planet_resource) => planet_resource,
                None => return Err("Failed to get the required resource from the planet"),
            };

            if planet_resource.amount < resource.1.amount {
                return Err("You don't have enough resources to build this building");
            }
        }

        for resource in &building_data.resource_cost {
            let planet_resource = match self.resources.get_mut(&resource.0) {
                Some(planet_resource) => planet_resource,
                None => return Err("Failed to get the required resource from the planet"),
            };

            planet_resource.amount -= resource.1.amount;
        }

        self.buildings.push(building);

        Ok(())
    }

    pub fn new(name: String, pos_x: u32, pos_y: u32) -> Self {
        let mut resources = HashMap::new();
        let credit_resource = Resource {
            resource_type: ResourceType::Credits,
            amount: 0,
        };
        let food_resource = Resource {
            resource_type: ResourceType::Food,
            amount: 0,
        };
        let mineral_resource = Resource {
            resource_type: ResourceType::Minerals,
            amount: 0,
        };
        resources.insert(ResourceType::Credits, credit_resource);
        resources.insert(ResourceType::Food, food_resource);
        resources.insert(ResourceType::Minerals, mineral_resource);

        let mut original_fleet = HashMap::new();
        let fleet_name = format!("{}{}", name, String::from(" fleet"));
        original_fleet.insert(fleet_name, Fleet { ships: vec![] });

        Self {
            name,
            buildings: vec![],
            military: original_fleet,
            resources,
            position: Position { x: pos_x, y: pos_y },
        }
    }
}

impl Tickable for Planet {
    fn tick(&mut self) -> TickResult {
        TickResult::None
    }
}
