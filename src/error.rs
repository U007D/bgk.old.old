use roll_amt::Error as RollAmtError;
use std::{ffi::OsString, fmt, option::NoneError};
use super::*;

#[derive(Clone, Debug, Fail, PartialEq, PartialOrd)]
pub enum Error {
    InvalidUtf8Arg(OsString),
    OptionNone,
    RollAmt(RollAmtError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::InvalidUtf8Arg(ref os_string) => format!("{}: {:?}", MSG_ERR_INVALID_UTF8_ARG, os_string),
            Error::OptionNone => MSG_ERR_NONE_ERROR.to_string(),
            Error::RollAmt(ref err) => format!("{}: {:?}", MSG_ERR_INVALID_ROLL_AMT, err),
        })
    }
}

impl From<OsString> for Error {
    fn from(err: OsString) -> Self {
        Error::InvalidUtf8Arg(err)
    }
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::OptionNone
    }
}

impl From<RollAmtError> for Error {
    fn from(err: RollAmtError) -> Self {
        Error::RollAmt(err)
    }
}
