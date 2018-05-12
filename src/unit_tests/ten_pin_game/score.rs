use game::Game;
use ten_pin_game::TenPinGame;
use rspec::{run, given};

#[derive(Clone, Default, Debug)]
struct Env {
    game: TenPinGame,
}

impl Env {
    fn new() -> Self {
        Self {
            game: TenPinGame::new(),
        }
    }
}

#[test]
fn tests() {
    run(&given("a fresh Game", Env::new(), |ctx| {
        let expected_result = 0_u16;
        ctx.then("the score should be 0", move |env| {
            assert!(env.game.score() == expected_result);
        });
    }));
}

