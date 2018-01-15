#![feature(termination_trait, nll, use_nested_groups)]

extern crate polish;

#[cfg(test)]
mod unit_tests;

struct Rolls;

impl Rolls {
    fn score(&self) -> u8 {
        0
    }
}

struct Game;

impl Game {
    fn new() -> Self {
        Game {}
    }

    fn roll(&self, roll: u8) -> Option<Rolls> {
        match roll <= 10 {
            true => Some(Rolls {}),
            false => None,
        }
    }
}

fn main() {

}
