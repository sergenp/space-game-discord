mod game;
use std::collections::HashMap;

use game::{
    build_tick_handler::BuildingTickRequest,
    buildings::{Building, BuildingData, ResourceBuilding},
    planet::Planet,
    resource::{Resource, ResourceType},
};
use mediator_sys::{
    builder::{BuilderFlow, BuilderInternal},
    synchronous::basic::{BasicMediator, SyncMediatorInternalHandle},
};

use crate::game::tickable::TickResult;

fn main() {
    let mut planet_1 = Planet::new(String::from("planet_1"), 0, 0);

    let mut create_type: HashMap<ResourceType, Resource> = HashMap::new();
    create_type.insert(
        ResourceType::Minerals,
        Resource {
            resource_type: ResourceType::Minerals,
            amount: 10,
        },
    );

    let mut resource_costs: HashMap<ResourceType, Resource> = HashMap::new();
    resource_costs.insert(
        ResourceType::Credits,
        Resource {
            resource_type: ResourceType::Credits,
            amount: 0,
        },
    );
    let resource_building: ResourceBuilding = ResourceBuilding {
        building: BuildingData {
            name: String::from("ResourceBuildingName"),
            resource_cost: resource_costs,
            level: 1,
        },
        create_type,
    };

    planet_1
        .build_building(Building::ResourceBuilding(resource_building))
        .unwrap();

    println!(
        "{:?}",
        planet_1.get_resource(ResourceType::Minerals).unwrap()
    );

    let mediator: BasicMediator<TickResult> = BasicMediator::<TickResult>::builder().build();
    mediator.send(BuildingTickRequest {
        planet: &mut planet_1,
    });

    println!(
        "{:?}",
        planet_1.get_resource(ResourceType::Minerals).unwrap()
    );
}
