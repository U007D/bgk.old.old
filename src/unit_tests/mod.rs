extern crate hesl;

use rspec::{given, run};
use di::Container;
use Game;

#[test]
fn tests() {
    run(&given("a game", (), |ctx| {
        let sut = Game::new();
        let expected_result = 0;

        ctx.then("the score should be 0", |_| {
            assert!(sut.score() == expected_result);
        });
    }));

    run(&given("a game where one gutterball has been rolled", (), |ctx| {
        let sut = Game::new();
        let expected_result = 0;

        ctx.when("the score is queried", |ctx| {
            let result = sut.score();

            ctx.then("", move |_| {
                assert!(result == expected_result);
            });
        });
    }));
}

