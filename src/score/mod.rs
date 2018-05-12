mod ten_pin_score;

type Result<T> = ::std::result::Result<T, Box<ScoreError>>;

trait Score {
    type Rolls;

    fn score(&self, rolls: &Self::Rolls) -> Result<i32>;
}

trait ScoreError {}
