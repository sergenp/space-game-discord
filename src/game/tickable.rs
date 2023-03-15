use std::collections::HashMap;

use super::military::MilitaryCreation;
use super::resource::{Resource, ResourceType};

pub trait Tickable {
    fn tick(&mut self) -> TickResult;
}

#[derive(Debug, Clone)]
pub enum TickResult {
    ResourceResult(HashMap<ResourceType, Resource>),
    MilitaryBuildResult(MilitaryCreation),
    None,
}
