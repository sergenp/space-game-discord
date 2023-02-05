pub struct MilitaryCreation {
    name: String,
    level: u32,
    attack: u32,
    defence: u32,
    military_type: MiltaryCreationTypes,
}

pub enum MiltaryCreationTypes {
    Ship,
    DefenceTurret,
}

impl MilitaryCreation {
    pub fn new(
        name: String,
        level: u32,
        attack: u32,
        defence: u32,
        military_type: MiltaryCreationTypes,
    ) -> Self {
        Self {
            name,
            level,
            attack,
            defence,
            military_type,
        }
    }
}
