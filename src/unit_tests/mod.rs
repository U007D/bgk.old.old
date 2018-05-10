extern crate hesl;

mod game;
mod mocks;

use game::Game;
use hesl::iter::single;
use self::mocks::*;
use rspec::{given, run};
