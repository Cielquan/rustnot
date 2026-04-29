#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Settings {
    pub sit_duration_as_min: u64,
    pub stand_duration_as_min: u64,
    pub start_stance: Stance,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sit_duration_as_min: 45,
            stand_duration_as_min: 20,
            start_stance: Stance::default(),
        }
    }
}

impl Settings {
    pub fn get_duration_for_stance(&self, stance: &Stance) -> u64 {
        match stance {
            Stance::Sitting => self.sit_duration_as_min,
            Stance::Standing => self.stand_duration_as_min,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Stance {
    #[default]
    Sitting,
    Standing,
}

impl Stance {
    pub fn inverted(current: Stance) -> Self {
        match current {
            Stance::Sitting => Stance::Standing,
            Stance::Standing => Stance::Sitting,
        }
    }
}
