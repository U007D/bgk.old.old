use roll_amt::RollAmt;

#[derive(Debug, Default, Clone)]
pub struct Game {
    score: u16,
}

impl Game {
    pub fn roll(&mut self, amt: RollAmt) -> &mut Self {
        self
    }

    pub fn score(&self) -> u16 {
        0
    }
}
