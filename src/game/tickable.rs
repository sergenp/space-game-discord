use std::collections::HashMap;

use super::military::MilitaryCreationTypes;
use super::resource::{Resource, ResourceType};

pub trait Tickable {
    fn tick(&mut self) -> TickResult;
}

#[derive(Debug, Clone)]
pub enum TickResult {
    ResourceResult(HashMap<ResourceType, Resource>),
    MilitaryBuildResult(MilitaryCreationTypes),
    None,
}
