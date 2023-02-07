use super::military::MilitaryCreation;
use super::resource::Resource;

pub trait Tickable {
    fn tick(&mut self) -> TickResult;
}

pub enum TickResult {
    ResourceResult(Vec<Resource>),
    MilitaryBuildResult(MilitaryCreation),
    None,
}
