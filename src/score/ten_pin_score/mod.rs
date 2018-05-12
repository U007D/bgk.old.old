mod error;
#[cfg(test)]
mod unit_tests;

pub use self::error::Error;
use roll_amt::RollAmt;
use rspec::{given, run};
use score::{Result, Score, ScoreError};

#[derive(Clone, Debug, Default)]
pub struct TenPinScore {}

impl Score for TenPinScore {
    type Rolls = [RollAmt; 21];

    fn score(&self, rolls: &Self::Rolls) -> Result<i32>  {
        Ok(rolls.iter()
                .map(|amt| i32::from(amt.as_u8()))
                .sum())
    }
}
