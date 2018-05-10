use super::*;
use score_keeper_error::ScoreKeeperError;

use std::fmt;

#[derive(Clone, Debug, Fail, PartialEq, PartialOrd)]
pub enum TenPinError {
    InvalidRollValue(u8),
}

impl fmt::Display for TenPinError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            TenPinError::InvalidRollValue(val) => format!("{}: {:?}", ten_pin::MSG_ERR_INVALID_ROLL_VALUE, val),
        })
    }
}

impl ScoreKeeperError for TenPinError {}
