use super::*;

#[derive(Clone, Default, Debug)]
struct Environment {
    game: Game,
    expected_result: Option<u16>,
    expected_times_called: u8,
}

#[test]
fn tests() {
    run(&given("a fresh game", Environment::default(), |ctx| {
        ctx.before(|env| {
            *env = Environment::default();
        });

        ctx.when("a 4 ball is rolled", |ctx| {
            ctx.before(|env| {
                env.game.roll(4);
                env.expected_result = Some(4);
                env.expected_times_called = 1;
            });

            ctx.then("and the pins method should have been called once", |env| {
                assert!(env.mock_score_keeper.pins_times_called == 1);
            });

            ctx.then("pins should have been passed the value 4", |env| {
                assert!(env.mock_score_keeper.pins.iter()
                                                  .single() == env.expected_result);
            });
        });
    }));
}
