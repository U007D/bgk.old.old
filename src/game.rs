use roll_amt::RollAmt;

#[derive(Debug, Default, Clone)]
pub struct Game {
    score: u16,
}

impl Game {
    pub fn roll(&mut self, amt: RollAmt) -> &mut Self {
        self.score = u16::from(amt.as_u8());
        self
    }

    pub fn score(&self) -> u16 {
        self.score
    }
}
