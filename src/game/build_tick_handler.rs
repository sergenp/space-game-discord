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
        let buildings = req.planet.buildings.iter_mut();
        for building in buildings {
            let tick_result = building.tick();

            match tick_result {
                TickResult::ResourceResult(res) => {
                    Planet::add_resource(&mut req.planet.resources, res)
                }
                TickResult::MilitaryBuildResult(mil) => {
                    let planet_name = req.planet.name.clone();
                    let fleet_name = format!("{}{}", planet_name, String::from(" fleet"));
                    Planet::add_military(req.planet.military.get_mut(&fleet_name).unwrap(), mil)
                }
                TickResult::None => (),
            }
        }
    }
}
