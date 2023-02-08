use super::buildings::Building;
use super::resource::Resource;
use super::resource::ResourceType;
use super::tickable::TickResult;
use super::tickable::Tickable;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct Position {
    x: u32,
    y: u32,
}

pub struct Planet {
    buildings: Vec<Box<dyn Building>>,
    resources: HashSet<ResourceType>,
    mineral_resource : ResourceType,
    food_resource : ResourceType,
    credit_resource : ResourceType,
    position: Position,
}

impl Planet {
    pub fn add_resource(&mut self, resource: Resource) {
        let planet_resource = match self.resources.get_mut(&resource.resource_type) {
            Some(planet_resource) => planet_resource,
            None => panic!("No resource have been found in the planet for given resource type"),
        };
        *planet_resource += resource;
    }

    // -> Option<&Resource> 
    pub fn get_resource(&self, resource_type: ResourceType){
        for abc in ResourceType::iterator() {
            let a = self.resources.get(&abc).unwrap();
        }
    }

    pub fn build_building(&mut self, building: Box<dyn Building>) -> Result<(), &str> {
        let building_data = building.get_building_data();
        let building_resource_type = &building_data.resource_cost.resource_type;

        let resource_type = match self.resources.get_mut(&building_resource_type) {
            Some(resource_type) => resource_type,
            None => panic!("No resource have been found in the planet for given resource type"),
        };
        
        building.resource.mineral_cost

        if building_data.resource_cost.amount > resource_type.amount {
            Err("You don't have required amount to build this building.")
        } else {
            resource_type.amount -= building_data.resource_cost.amount;
            self.buildings.push(building);
            Ok(())
        }
    }
    pub fn new(pos_x: u32, pos_y: u32) -> Self {
        // let mut resources = HashSet::new();
        // resources.insert(ResourceType::Credits, credit_resource);
        // resources.insert(ResourceType::Food, food_resource);
        // resources.insert(ResourceType::Minerals, mineral_resource);


        Self {
            buildings: vec![],
            resources,
            position: Position { x: pos_x, y: pos_y },
        }
    }
}

impl Tickable for Planet {
    fn tick(&mut self) -> TickResult {
        for building in self.buildings.iter_mut() {
            match building.tick() {
                TickResult::ResourceResult(resources) => {
                    for abc in ResourceType::iterator() {
                        let a = self.resources.get(&abc).unwrap().to_owned();
                        a.0
                    }
                    for resource in resources {

                        // can't use self.add_resource here because we can't pass self to the add_resource function
                        // doing so would cause another mutable borrow
                        let resource_type = match self.resources.get_mut(&resource.resource_type) {
                            Some(resource_type) => resource_type,
                            None => panic!(
                                "No resource have been found in the planet for building the building."
                            ),
                        };

                        *resource_type += resource;
                    }
                }
                TickResult::MilitaryBuildResult(_military_creation) => todo!(),
                TickResult::None => (),
            };
        }
        TickResult::None
    }
}
