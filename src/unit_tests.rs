use polish::{test_case::{TestRunner, TestCaseStatus, TestCase, TEST_RUNNER_ATTRIBUTES}, logger::Logger};
#[allow(unused_imports)] use super::*;

#[test]
fn tests() {
    TestRunner::new().set_module_path(module_path!()).set_attributes(TEST_RUNNER_ATTRIBUTES.disable_final_stats | TEST_RUNNER_ATTRIBUTES.minimize_output).run_tests(vec![
        TestCase::new("Game::roll()", "rolling a gutterball yields a score of 0", Box::new(|_logger: &mut Logger| -> TestCaseStatus {
            // GIVEN a Game instance
            let game = Game::new();

            // WHEN a gutterball is rolled
            let result = game.roll(0);

            // THEN
            match result.score() == 0 {
                true => TestCaseStatus::PASSED,
                false => TestCaseStatus::FAILED,
            }
        })),
    ]);
}
