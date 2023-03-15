use mediator_sys::synchronous::basic::*;

use crate::game::resource::ResourceType;

use super::{buildings::Building, planet::Planet, tickable::TickResult};

pub struct BuildingTickRequest<'a> {
    pub planet: &'a mut Planet,
    pub building: Box<dyn Building>,
}

impl RequestHandler<BuildingTickRequest<'_>, TickResult> for BasicMediator<TickResult> {
    fn handle(&self, mut req: BuildingTickRequest) {
        let tick_result = req.building.tick();
        if let TickResult::ResourceResult(res) = tick_result {
            println!("Got {:?}", res);

            req.planet
                .add_resource(*res.get(&ResourceType::Minerals).unwrap())
        }
    }
}
