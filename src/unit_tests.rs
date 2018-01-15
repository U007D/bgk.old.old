use polish::{test_case::{TestRunner, TestCaseStatus, TestCase, TEST_RUNNER_ATTRIBUTES}, logger::Logger};
#[allow(unused_imports)] use super::*;

#[test]
fn tests() {
    TestRunner::new().set_module_path(module_path!()).set_attributes(TEST_RUNNER_ATTRIBUTES.disable_final_stats | TEST_RUNNER_ATTRIBUTES.minimize_output).run_tests(vec![
        TestCase::new("Game::roll()", "rolling a gutterball yields a score of 0", Box::new(|_logger: &mut Logger| -> TestCaseStatus {
            // GIVEN a Game instance
            let mut game = Game::new();

            // WHEN a gutterball is rolled
            let result = game.roll(0);

            // THEN
            match result.score() {
                Some(0) => TestCaseStatus::PASSED,
                _ => TestCaseStatus::FAILED,
            }
        })),
        TestCase::new("Game::roll()", "attempting to roll an illegal value of 11 fails", Box::new(|_logger: &mut Logger| -> TestCaseStatus {
            // GIVEN a Game instance
            let mut game = Game::new();

            // WHEN an illegal value is rolled
            let result = game.roll(11);

            // THEN
            match result.score() {
                None => TestCaseStatus::PASSED,
                Some(_) => TestCaseStatus::FAILED,
            }
        })),
        TestCase::new("Game::roll()", "rolling a 1 followed by a 1 yields a score of 2", Box::new(|_logger: &mut Logger| -> TestCaseStatus {
            // GIVEN a Game instance
            let mut game = Game::new();

            // WHEN two 1's are rolled
            let result = game.roll(1)
                             .roll(1);

            // THEN
            match result.score() {
                Some(2) => TestCaseStatus::PASSED,
                _ => TestCaseStatus::FAILED,
            }
        })),
        TestCase::new("Game::roll()", "attempting to roll an illegal frame of 1 + 10 fails", Box::new(|_logger: &mut Logger| -> TestCaseStatus {
            // GIVEN a Game instance
            let mut game = Game::new();

            // WHEN an illegal value is rolled
            let result = game.roll(1)
                             .roll(10);

            // THEN
            match result.score() {
                Some(1) => TestCaseStatus::PASSED,
                _ => TestCaseStatus::FAILED,
            }
        })),
    ]);
}