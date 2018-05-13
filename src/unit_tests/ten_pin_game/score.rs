use game::{Error as GameError, Game};
use rspec::{run, given};
use ten_pin_game::TenPinGame;

#[derive(Clone, Debug)]
struct Env {
    game: TenPinGame,
    result: Option<Result<<TenPinGame as Game>::Score, GameError>>,
}

impl Env {
    fn new() -> Self {
        Self {
            game: TenPinGame::new(),
            result: None,
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

    run(&given("a fresh Game", Env::new(), |ctx| {
        ctx.when("no balls are rolled", |ctx| {
            ctx.before(|env| {
                env.result = env.game.score(&[]);
            });
            let expected_result = 0_u16;

            ctx.then("the score should be 0", move |env| {
                assert!(env.game.score() == expected_result);
            });
        });
    }));

}

