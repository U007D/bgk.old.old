#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Game {
    score: u16,
}

impl Game {
    pub fn new() -> Self {
        Self {
            score: 0,
        }
    }

    pub fn roll(&self, val: u8) -> &Self {
        self
    }

    pub fn score(&self) -> u16 {
        0
    }
}
