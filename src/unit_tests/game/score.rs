use super::*;
use game::Game;
use roll_amt::RollAmt;

#[derive(Clone, Default, Debug)]
struct Env {
    game: Game,
}

#[test]
fn tests() {
    run(&given("an initialized Game instance", Env::default(), |ctx| {
        let expected_result = 0;

        ctx.then("with no rolls, score should be 0", move |env| {
            assert!(env.game.score() == expected_result);
        });

        ctx.when("a gutter ball is rolled", |ctx| {
            ctx.before(|env| {
                env.game.roll(RollAmt::new(0).unwrap());
            });
            let expected_result = 0;

            ctx.then("score should be 0", move |env| {
                assert!(env.game.score() == expected_result);
            });
        });

        ctx.when("1 pin is hit", |ctx| {
            ctx.before(|env| {
                env.game.roll(RollAmt::new(1).unwrap());
            });
            let expected_result = 1;

            ctx.then("score should be 1", move |env| {
                assert!(env.game.score() == expected_result);
            });
        });
    }));
}

