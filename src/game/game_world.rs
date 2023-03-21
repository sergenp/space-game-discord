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
        buildings::{Building, BuildingData, MilitaryBuilding, ResourceBuilding},
        game_world::GameWorld,
        military::{MilitaryCreation, MilitaryCreationTypes},
        planet::Planet,
        resource::{Resource, ResourceType},
        tickable::Tickable,
    };

    fn create_resource_map(
        credit_amount: i32,
        food_amount: i32,
        mineral_amount: i32,
    ) -> HashMap<ResourceType, Resource> {
        let mut resource = HashMap::new();

        resource.insert(
            ResourceType::Credits,
            Resource {
                resource_type: ResourceType::Credits,
                amount: credit_amount,
            },
        );
        resource.insert(
            ResourceType::Food,
            Resource {
                resource_type: ResourceType::Food,
                amount: food_amount,
            },
        );
        resource.insert(
            ResourceType::Minerals,
            Resource {
                resource_type: ResourceType::Minerals,
                amount: mineral_amount,
            },
        );

        resource
    }

    #[test]
    fn building_without_enough_resource_error() {
        let mut planet_1 = Planet::new(String::from("planet_1"), 0, 0);

        let create_type = create_resource_map(0, 0, 10);
        let resource_cost = create_resource_map(20, 0, 0);

        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("ResourceBuildingName"),
                resource_cost,
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
        let mut planet_1: Planet = Planet::new(String::from("planet_1"), 0, 0);

        let resource_to_add = create_resource_map(20, 0, 0);
        Planet::add_resource(&mut planet_1.resources, resource_to_add);

        let create_type = create_resource_map(0, 0, 10);
        let resource_cost = create_resource_map(20, 0, 0);

        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("ResourceBuildingName"),
                resource_cost,
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
        let mut planet_1: Planet = Planet::new(String::from("planet_1"), 0, 0);

        let resource_to_add = create_resource_map(0, 5, 0);
        Planet::add_resource(&mut planet_1.resources, resource_to_add);

        let create_type = create_resource_map(0, 0, 10);
        let resource_cost = create_resource_map(0, 5, 0);

        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("BuildingThatCostsFood"),
                resource_cost,
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
        let mut planet_1: Planet = Planet::new(String::from("planet_1"), 0, 0);

        let resource_to_add = create_resource_map(0, 5, 0);
        Planet::add_resource(&mut planet_1.resources, resource_to_add);

        let create_type = create_resource_map(5, 0, 10);
        let resource_cost = create_resource_map(0, 5, 0);

        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("BuildingThatCostsFood"),
                resource_cost,
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
        let mut planet_1: Planet = Planet::new(String::from("planet_1"), 0, 0);

        let resource_to_add = create_resource_map(20, 5, 0);

        Planet::add_resource(&mut planet_1.resources, resource_to_add);

        let create_type = create_resource_map(-5, 0, 10);

        let resource_costs = create_resource_map(0, 5, 0);

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

    #[test]
    fn building_with_military_creation_test() {
        let planet_1: Planet = Planet::new(String::from("planet_1"), 0, 0);
        let mut world = GameWorld::new(vec![planet_1]);

        let mut planet_1 = &mut world.planets[0];

        let resource_to_add = create_resource_map(1000, 1000, 1000);

        let resource_cost_for_building = create_resource_map(0, 0, 0);

        let military_cost_per_tick = create_resource_map(-10, -10, -10);

        Planet::add_resource(&mut planet_1.resources, resource_to_add);

        let create_type = MilitaryCreation {
            attack: 5,
            defence: 5,
            level: 1,
            name: String::from("MilitaryCreation1"),
        };

        let military_building: MilitaryBuilding = MilitaryBuilding {
            building: BuildingData {
                name: String::from("MilitaryBuilding"),
                resource_cost: resource_cost_for_building,
                level: 1,
            },
            create_type: MilitaryCreationTypes::Ship(create_type),
            military_cost_per_tick,
            progress_required_to_create_military: 20,
            current_progress: 0,
            progress_increase_per_tick: 10,
        };

        assert!(planet_1
            .build_building(Building::MilitaryBuilding(military_building))
            .is_ok());

        world.tick();

        // we had 1000 credits, now we will have 990 because of militay_cost_per_tick
        assert!(
            world.planets[0]
                .get_resource(ResourceType::Credits)
                .unwrap()
                .amount
                == 990
        );
        // after the second tick, we expect our military creation to complete
        world.tick();
        let fleet_name = format!("{}{}", world.planets[0].name.clone(), " fleet");
        let fleet = world.planets[0].military.get(&fleet_name).unwrap();

        assert_eq!(fleet.ships.len(), 1);
    }
}
