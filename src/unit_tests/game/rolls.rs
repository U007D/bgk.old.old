use game::Game;
use rspec::{given, run};

#[derive(Clone, Default, Debug)]
struct Env {
    game: Game,
}

impl Env {
    fn new() -> Self {
        Self {
            game: Game::new(),
        }
    }
}

#[test]
fn tests() {
    run(&given("a fresh Game", Env::new(), |ctx| {
        ctx.when("no balls are rolled", |ctx| {
            ctx.before(|env| {
                env.game.rolls(&[]);
            });
            let expected_result = 0_u16;

            ctx.then("the score should be 0", move |env| {
                assert!(env.game.score() == expected_result);
            });
        });
    }));
}

