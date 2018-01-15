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

    fn roll(&self, _: u8) -> Rolls {
        Rolls {}
    }
}

fn main() {

}
