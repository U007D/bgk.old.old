use super::*;

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
    run(&given("a newly-initialized Game", Env::new(), |ctx| {
        let expected_result = 0_u16;
        ctx.then("the score should be 0", |env| {
            assert!(env.game.score() == expected_result);
        });
    }));
}

