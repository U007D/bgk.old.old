use super::*;
use roll_amt::RollAmt;

#[derive(Clone, Default, Debug)]
struct Env {
    score: TenPinScore,
    result: Option<Result<usize>>,
}

#[test]
fn tests() {
    run(&given("an initialized Score instance", Env::default(), |ctx| {
        ctx.when("no balls are rolled", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.score.roll(&[RollAmt::new(0).unwrap()]));
            });
            let expected_result = Ok(0_usize);

            ctx.then("the score should be 0", |env| {
                assert!(env.result == Some(expected_result));
            });
        });

        ctx.when("a gutter ball is rolled", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.score.roll(&[RollAmt::new(0).unwrap()]));
            });
            let expected_result = Ok(0_usize);

            ctx.then("score should be 0", move |env| {
                assert!(env.result == Some(expected_result));
            });
        });

        ctx.when("1 pin is hit", |ctx| {
            ctx.before(|env| {
                env.result = Some(env.game.roll([RollAmt::new(1).unwrap()]));
            });
            let expected_result = Ok(1_usize);

            ctx.then("score should be 1", move |env| {
                assert!(env.score.score() == Some(expected_result));
            });
        });
    }));
}

