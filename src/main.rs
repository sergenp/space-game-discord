mod game;
use std::collections::HashMap;

use game::{
    build_tick_handler::BuildingTickRequest,
    buildings::{BuildingData, ResourceBuilding},
    planet::Planet,
    resource::{Resource, ResourceType},
};
use mediator_sys::{
    builder::{BuilderFlow, BuilderInternal},
    synchronous::basic::{BasicMediator, SyncMediatorInternalHandle, SyncMediatorInternalNext},
};

use crate::game::tickable::TickResult;

fn main() {
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

    let mediator = BasicMediator::<TickResult>::builder()
        .add_listener(move |ev| {
            if let TickResult::None = ev {
                println!("Ignored some Message")
            }
        })
        .add_listener(move |ev| {
            if let TickResult::ResourceResult(res) = ev {
                println!("Got {:?}", res);
            }
        })
        .build();

    println!(
        "{:?}",
        planet_1.get_resource(ResourceType::Minerals).unwrap()
    );
    mediator.send(BuildingTickRequest {
        planet: &mut planet_1,
        building: Box::new(resource_building),
    });
    print!(
        "{:?}",
        planet_1.get_resource(ResourceType::Minerals).unwrap()
    );
}
