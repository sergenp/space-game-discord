use mediator_sys::synchronous::basic::*;

use super::{
    planet::Planet,
    tickable::{TickResult, Tickable},
};

pub struct BuildingTickRequest<'a> {
    pub planet: &'a mut Planet,
}

impl RequestHandler<BuildingTickRequest<'_>, TickResult> for BasicMediator<TickResult> {
    fn handle(&self, req: BuildingTickRequest) {
        for building in req.planet.buildings.iter_mut() {
            let tick_result = building.tick();
            match tick_result {
                TickResult::ResourceResult(res) => {
                    Planet::add_resource(&mut req.planet.resources, res)
                }
                _ => (),
            }
        }
    }
}
