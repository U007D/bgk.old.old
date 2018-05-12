use std::fmt;
use super::*;

#[derive(Clone, Debug, Fail, PartialEq, PartialOrd)]
pub enum Error {
    InvalidRollAmt(Option<u64>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::InvalidRollAmt(ref amt) => format!("{}: {:?}", MSG_ERR_INVALID_ROLL_AMT, amt),
        })
    }
}
