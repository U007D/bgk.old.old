use super::*;
use score_keeper_error::ScoreKeeperError;

use std::{ffi::OsString, fmt};

#[derive(Clone, Debug, Fail, PartialEq, PartialOrd)]
pub enum Error {
    InvalidUtf8Arg(OsString),
    ScoreKeeper(Box<ScoreKeeperError>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::InvalidUtf8Arg(ref os_string) => format!("{}: {:?}", MSG_ERR_INVALID_UTF8_ARG, os_string),
            Error::ScoreKeeper(ref err) => format!("{}: {}", MSG_ERR_INTERNAL_SCORE_KEEPER, err),
        })
    }
}

impl From<OsString> for Error {
    fn from(err: OsString) -> Self {
        Error::InvalidUtf8Arg(err)
    }
}

impl<E> From<E> for Error where E: ScoreKeeperError {
    fn from(err: E) -> Self {
        Error::ScoreKeeper(Box::new(err))
    }
}
