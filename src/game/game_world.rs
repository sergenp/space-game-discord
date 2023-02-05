use super::planet::Planet;
use super::tickable::{TickResult, Tickable};

pub struct GameWorld {
    pub planets: Vec<Planet>,
}

impl Tickable for GameWorld {
    fn tick(&mut self) -> TickResult {
        for planet in self.planets.iter_mut() {
            planet.tick();
        }
        TickResult::None
    }
}

#[cfg(test)]
mod tests {
    use crate::game::{
        buildings::{BuildingData, ResourceBuilding},
        planet::Planet,
        resource::{Resource, ResourceType},
        tickable::Tickable,
    };

    use super::GameWorld;

    #[test]
    fn building_without_enough_resource_error() {
        let mut planet_1 = Planet::new(0, 0);
        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("ResourceBuildingName"),
                resource_cost: Resource {
                    resource_type: ResourceType::Credits,
                    amount: 20,
                },
                level: 1,
            },
            create_type: Resource {
                resource_type: ResourceType::Minerals,
                amount: 10,
            },
        };
        assert!(planet_1
            .build_building(Box::new(resource_building))
            .is_err(),);
    }

    #[test]
    fn building_with_enough_resource_success() {
        let mut planet_1: Planet = Planet::new(0, 0);
        planet_1.add_resource(Resource {
            resource_type: ResourceType::Credits,
            amount: 20,
        });
        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("ResourceBuildingName"),
                resource_cost: Resource {
                    resource_type: ResourceType::Credits,
                    amount: 20,
                },
                level: 1,
            },
            create_type: Resource {
                resource_type: ResourceType::Minerals,
                amount: 10,
            },
        };
        assert!(planet_1.build_building(Box::new(resource_building)).is_ok());
        assert!(planet_1.get_resource(ResourceType::Credits).unwrap().amount == 0);
    }
    #[test]
    fn resource_building_tick_test_with_planet() {
        let mut planet_1: Planet = Planet::new(0, 0);
        // planets start with 0 resources
        planet_1.add_resource(Resource {
            resource_type: ResourceType::Food,
            amount: 5,
        });

        let resource_building: ResourceBuilding = ResourceBuilding {
            building: BuildingData {
                name: String::from("BuildingThatCostsFood"),
                resource_cost: Resource {
                    resource_type: ResourceType::Food,
                    amount: 5,
                },
                level: 1,
            },
            // generate 10 minerals per tick
            create_type: Resource {
                resource_type: ResourceType::Minerals,
                amount: 10,
            },
        };
        // building the building should be ok, since we have 5 food
        // and the resource cost of the building is 5 food
        assert!(planet_1.build_building(Box::new(resource_building)).is_ok());
        // if building is ok food should decrease to 0
        assert!(planet_1.get_resource(ResourceType::Food).unwrap().amount == 0);
        // assert we have no minerals
        assert!(
            planet_1
                .get_resource(ResourceType::Minerals)
                .unwrap()
                .amount
                == 0
        );
        // tick the planet, it ticks all the buildings it has, ticking the resource building
        // should give us create_type resource, which is planet.Minerals + 10 per tick
        planet_1.tick();
        // first tick should give us 0+10 minerals
        assert!(
            planet_1
                .get_resource(ResourceType::Minerals)
                .unwrap()
                .amount
                == 10
        );
        planet_1.tick();
        // second tick should give us 10+10 minerals
        assert!(
            planet_1
                .get_resource(ResourceType::Minerals)
                .unwrap()
                .amount
                == 20
        );
    }
}
