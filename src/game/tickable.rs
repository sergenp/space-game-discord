use std::collections::HashSet;

use super::military::MilitaryCreation;
use super::resource::Resource;

pub trait Tickable {
    fn tick(&mut self) -> TickResult;
}

pub enum TickResult {
    ResourceResult(HashSet<Resource>),
    MilitaryBuildResult(MilitaryCreation),
    None,
}
