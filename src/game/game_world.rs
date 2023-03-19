use mediator_sys::builder::{BuilderFlow, BuilderInternal};
use mediator_sys::synchronous::basic::{BasicMediator, SyncMediatorInternalHandle};

use super::build_tick_handler::BuildingTickRequest;
use super::planet::Planet;
use super::tickable::{TickResult, Tickable};

pub struct GameWorld {
    pub planets: Vec<Planet>,
    pub tick_mediator: BasicMediator<TickResult>,
}

impl GameWorld {
    pub fn new(planets: Vec<Planet>) -> Self {
        Self {
            planets,
            tick_mediator: BasicMediator::<TickResult>::builder().build(),
        }
    }
}

impl Tickable for GameWorld {
    fn tick(&mut self) -> TickResult {
        for planet in self.planets.iter_mut() {
            self.tick_mediator.send(BuildingTickRequest { planet });
        }
        TickResult::None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::game::{
        buildings::{Building, BuildingData, ResourceBuilding},
        game_world::GameWorld,
        planet::Planet,
        resource::{Resource, ResourceType},
        tickable::Tickable,
    };

    #[test]
    fn building_without_enough_resource_error() {
        let mut planet_1 = Planet::new(0, 0);
        let mut create_type: HashMap<ResourceType, Resource> = HashMap::new();
        create_type.insert(
            ResourceType::Minerals,
            Resource {
                resource_type: ResourceType::Minerals,
                amount: 10,
            },
        );
        let resource_cost = Resource {
            resource_type: ResourceType::Credits,
            amount: 20,
        };

        let mut resource_costs: HashMap<ResourceType, Resource> = HashMap::new();
        resource_costs.insert(ResourceType::Credits, resource_cost);
        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("ResourceBuildingName"),
                resource_cost: resource_costs,
                level: 1,
            },
            create_type,
        };

        assert!(planet_1
            .build_building(Building::ResourceBuilding(resource_building))
            .is_err());
    }

    #[test]
    fn building_with_enough_resource_success() {
        let mut planet_1: Planet = Planet::new(0, 0);
        let mut resource_to_add: HashMap<ResourceType, Resource> = HashMap::new();
        resource_to_add.insert(
            ResourceType::Credits,
            Resource {
                resource_type: ResourceType::Credits,
                amount: 20,
            },
        );
        Planet::add_resource(&mut planet_1.resources, resource_to_add);

        let mut create_type: HashMap<ResourceType, Resource> = HashMap::new();
        create_type.insert(
            ResourceType::Minerals,
            Resource {
                resource_type: ResourceType::Minerals,
                amount: 10,
            },
        );

        let resource_cost = Resource {
            resource_type: ResourceType::Credits,
            amount: 20,
        };
        let mut resource_costs: HashMap<ResourceType, Resource> = HashMap::new();
        resource_costs.insert(ResourceType::Credits, resource_cost);

        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("ResourceBuildingName"),
                resource_cost: resource_costs,
                level: 1,
            },
            create_type,
        };
        assert!(planet_1
            .build_building(Building::ResourceBuilding(resource_building))
            .is_ok());
        assert!(planet_1.get_resource(ResourceType::Credits).unwrap().amount == 0);
    }

    #[test]
    fn resource_building_tick_test_with_planet() {
        let mut planet_1: Planet = Planet::new(0, 0);
        // planets start with 0 resources
        let mut resource_to_add: HashMap<ResourceType, Resource> = HashMap::new();
        resource_to_add.insert(
            ResourceType::Food,
            Resource {
                resource_type: ResourceType::Food,
                amount: 5,
            },
        );

        Planet::add_resource(&mut planet_1.resources, resource_to_add);

        let mut create_type: HashMap<ResourceType, Resource> = HashMap::new();
        create_type.insert(
            ResourceType::Minerals,
            Resource {
                resource_type: ResourceType::Minerals,
                amount: 10,
            },
        );

        let resource_cost = Resource {
            resource_type: ResourceType::Food,
            amount: 5,
        };

        let mut resource_costs: HashMap<ResourceType, Resource> = HashMap::new();
        resource_costs.insert(ResourceType::Food, resource_cost);

        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("BuildingThatCostsFood"),
                resource_cost: resource_costs,
                level: 1,
            },
            // generate 10 minerals per tick
            create_type,
        };
        // building the building should be ok, since we have 5 food
        // and the resource cost of the building is 5 food
        let mut world = GameWorld::new(vec![planet_1]);

        assert!(world.planets[0]
            .build_building(Building::ResourceBuilding(resource_building))
            .is_ok());

        // if building is ok food should decrease to 0
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Food)
                .unwrap()
                .amount
                == 0
        );
        // assert we have no minerals
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Minerals)
                .unwrap()
                .amount
                == 0
        );
        // tick the planet, it ticks all the buildings it has, ticking the resource building
        // should give us create_type resource, which is planet.Minerals + 10 per tick
        world.tick();
        // first tick should give us 0+10 minerals
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Minerals)
                .unwrap()
                .amount
                == 10
        );
        world.tick();
        // second tick should give us 10+10 minerals
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Minerals)
                .unwrap()
                .amount
                == 20
        );
    }

    #[test]
    fn multi_resource_building_tick_test_with_planet() {
        let mut planet_1: Planet = Planet::new(0, 0);
        // planets start with 0 resources
        let mut resource_to_add: HashMap<ResourceType, Resource> = HashMap::new();
        resource_to_add.insert(
            ResourceType::Food,
            Resource {
                resource_type: ResourceType::Food,
                amount: 5,
            },
        );

        Planet::add_resource(&mut planet_1.resources, resource_to_add);

        let mut create_type: HashMap<ResourceType, Resource> = HashMap::new();
        create_type.insert(
            ResourceType::Minerals,
            Resource {
                resource_type: ResourceType::Minerals,
                amount: 10,
            },
        );
        create_type.insert(
            ResourceType::Credits,
            Resource {
                resource_type: ResourceType::Credits,
                amount: 5,
            },
        );

        let resource_cost = Resource {
            resource_type: ResourceType::Food,
            amount: 5,
        };

        let mut resource_costs: HashMap<ResourceType, Resource> = HashMap::new();
        resource_costs.insert(ResourceType::Food, resource_cost);

        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("BuildingThatCostsFood"),
                resource_cost: resource_costs,
                level: 1,
            },
            // generate 10 minerals per tick and 5 credits per tick
            create_type,
        };
        let mut world = GameWorld::new(vec![planet_1]);

        // building the building should be ok, since we have 5 food
        // and the resource cost of the building is 5 food
        assert!(world.planets[0]
            .build_building(Building::ResourceBuilding(resource_building))
            .is_ok());
        // if building is ok food should decrease to 0
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Food)
                .unwrap()
                .amount
                == 0
        );
        // assert we have no minerals
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Minerals)
                .unwrap()
                .amount
                == 0
        );
        // and no credits
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Credits)
                .unwrap()
                .amount
                == 0
        );
        // tick the planet, it ticks all the buildings it has, ticking the resource building
        // should give us create_type resource, which is planet.Minerals + 10 per tick and planet.Credits + 5 per tick
        world.tick();
        // first tick should give us 0+10 minerals
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Minerals)
                .unwrap()
                .amount
                == 10
        );
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Credits)
                .unwrap()
                .amount
                == 5
        );
        world.tick();
        // second tick should give us 10+10 minerals, 5+5 credits
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Minerals)
                .unwrap()
                .amount
                == 20
        );
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Credits)
                .unwrap()
                .amount
                == 10
        );
    }

    #[test]
    fn multi_resource_minus_amount_building_tick_test_with_planet() {
        let mut planet_1: Planet = Planet::new(0, 0);
        let mut resource_to_add: HashMap<ResourceType, Resource> = HashMap::new();
        resource_to_add.insert(
            ResourceType::Food,
            Resource {
                resource_type: ResourceType::Food,
                amount: 5,
            },
        );
        resource_to_add.insert(
            ResourceType::Credits,
            Resource {
                resource_type: ResourceType::Credits,
                amount: 20,
            },
        );

        Planet::add_resource(&mut planet_1.resources, resource_to_add);

        let mut create_type: HashMap<ResourceType, Resource> = HashMap::new();
        create_type.insert(
            ResourceType::Minerals,
            Resource {
                resource_type: ResourceType::Minerals,
                amount: 10,
            },
        );
        create_type.insert(
            ResourceType::Credits,
            Resource {
                resource_type: ResourceType::Credits,
                amount: -5,
            },
        );

        let resource_cost = Resource {
            resource_type: ResourceType::Food,
            amount: 5,
        };

        let mut resource_costs: HashMap<ResourceType, Resource> = HashMap::new();
        resource_costs.insert(ResourceType::Food, resource_cost);

        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("BuildingThatCostsFood"),
                resource_cost: resource_costs,
                level: 1,
            },
            // generate 10 minerals per tick and 5 credits per tick
            create_type,
        };

        let mut world = GameWorld::new(vec![planet_1]);

        // building the building should be ok, since we have 5 food
        // and the resource cost of the building is 5 food
        assert!(world.planets[0]
            .build_building(Building::ResourceBuilding(resource_building))
            .is_ok());

        // tick the planet, it ticks all the buildings it has, ticking the resource building
        // should give us create_type resource, which is planet.Minerals + 10 per tick and planet.Credits - 5 per tick
        world.tick();
        // first tick should give us 0+10 minerals
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Minerals)
                .unwrap()
                .amount
                == 10
        );
        // we had 20 credits, now we will have 15
        println!(
            "{}",
            world.planets[0]
                .get_resource(ResourceType::Credits)
                .unwrap()
                .amount
        );
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Credits)
                .unwrap()
                .amount
                == 15
        );

        world.tick(); // second tick should give us 10+10 minerals, 15-5 credits

        assert!(
            world.planets[0]
                .get_resource(ResourceType::Minerals)
                .unwrap()
                .amount
                == 20
        );
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Credits)
                .unwrap()
                .amount
                == 10
        );
    }
}
