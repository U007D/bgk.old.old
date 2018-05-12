#[derive(Clone, Default, Debug)]
pub struct Game {}

impl Game {
    pub fn new() -> Self {
        Self {}
    }

    pub fn score(&self) -> u16 {
        0
    }
}
