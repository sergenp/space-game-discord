#[derive(Debug, Clone)]
pub struct MilitaryCreation {
    name: String,
    level: u32,
    attack: u32,
    defence: u32,
}

#[derive(Debug, Clone)]
pub enum MilitaryCreationTypes {
    Ship(MilitaryCreation),
    DefenceTurret(MilitaryCreation),
}

impl MilitaryCreation {
    pub fn new(name: String, level: u32, attack: u32, defence: u32) -> Self {
        Self {
            name,
            level,
            attack,
            defence,
        }
    }
}
