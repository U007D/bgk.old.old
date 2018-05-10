extern crate hesl;

use rspec::{given, run};
use di::Container;
use Game;

#[test]
fn tests() {
    run(&given("a game where no balls have been rolled", (), |ctx| {
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

