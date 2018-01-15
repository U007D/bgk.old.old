#![feature(termination_trait, nll, use_nested_groups)]

extern crate polish;

#[cfg(test)]
mod unit_tests;

const MAX_PINS_PER_FRAME: u8 = 10;
const ROLLS_PER_FRAME: usize = 2;
const AWAITING_1ST_BALL: usize = 0;
const AWAITING_2ND_BALL: usize = 1;

pub struct Game {
    rolls: Vec<u8>,
}

impl Game {
    pub fn new() -> Self {
        Game { rolls: Vec::<u8>::new() }
    }

    pub fn roll(&mut self, roll: u8) -> &mut Self {
        let pins_this_frame = match self.rolls.len() % ROLLS_PER_FRAME {
            AWAITING_1ST_BALL => 0,
            AWAITING_2ND_BALL => self.rolls[self.rolls.len() - 1],
            _ => panic!("Internal error: ROLLS_PER_FRAME != 2"),
        };

        if roll <= MAX_PINS_PER_FRAME - pins_this_frame {
            self.rolls.push(roll);
        }

        self
    }

    pub fn score(&self) -> Option<u8> {
        match self.rolls.len() {
            0 => None,
            _ => Some(self.rolls.iter().sum())
        }
    }
}

fn main() {

}
