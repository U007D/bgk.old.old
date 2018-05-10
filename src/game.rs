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

    pub fn roll(&mut self, val: u8) -> &Self {
        self.score = u16::from(val);
        self
    }

    pub fn score(&self) -> u16 {
        self.score
    }
}
