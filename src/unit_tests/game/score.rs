use super::*;
use game::Game;

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

//        ctx.when("", |ctx| {
//            ctx.before(|env| {});
//
//            ctx.then("", |env| {
//            });
//        });
    }));
}

