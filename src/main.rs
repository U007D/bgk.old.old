#![feature(termination_trait, nll, use_nested_groups)]

extern crate polish;

#[cfg(test)]
mod unit_tests;

struct Game {
    rolls: Vec<u8>,
}

impl Game {
    fn new() -> Self {
        Game { rolls: Vec::<u8>::new() }
    }

    fn roll(&mut self, roll: u8) -> &mut Self {
        if roll <= 10 {
            self.rolls.push(roll);
        }
        self
    }

    fn score(&self) -> Option<u8> {
        match self.rolls.len() {
            0 => None,
            _ => Some(self.rolls.iter().sum())
        }
    }
}

fn main() {

}
