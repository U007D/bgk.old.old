use consts::*;
use std::fmt;
use super::{Score, ScoreError, TenPinScore};

#[derive(Clone, Debug, Fail, PartialEq, PartialOrd)]
pub enum Error {
    InvalidRoll(<TenPinScore as Score>::Rolls, usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::InvalidRoll(ref rolls, ref index) => format!("{}: {:?}[{}]", MSG_ERR_INVALID_ROLL, rolls, index),
        })
    }
}

impl ScoreError for Error {}
