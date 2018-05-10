use super::*;

#[derive(Clone, Default, Debug)]
struct Environment {
    game: Game,
    expected_result: u16,
}

#[test]
fn tests() {
    run(&given("a fresh game", Environment::default(), |ctx| {
        ctx.before(|env| {
            *env = Environment::default();
        });

        ctx.then("the score should be 0", |env| {
            assert!(env.game.score() == env.expected_result);
        });

        ctx.when("a gutter ball is rolled", |ctx| {
            ctx.before(|env| {
                env.game.roll(0);
                env.expected_result = 0;
            });

            ctx.then("the score should be 0", |env| {
                assert!(env.game.score() == env.expected_result);
            });
        });

        ctx.when("a pin is hit", |ctx| {
            ctx.before(|env| {
                env.game.roll(1);
                env.expected_result = 1;
            });

            ctx.then("the score should be 1", |env| {
                assert!(env.game.score() == env.expected_result);
            });
        });
    }));
}

