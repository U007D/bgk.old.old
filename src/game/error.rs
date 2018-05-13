use consts::*;
use std::fmt;

#[derive(Clone, Debug, Fail, PartialEq, PartialOrd)]
pub enum Error {
    UnknownPinId(PinId, PinList, usize),
    PinIdSpecifiedTwiceInSameFrame(PinId, PinList, usize, usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::UnknownPinId(ref id, ref list, ref idx) =>
                format!("{}: {:?}: {:?}[{}]", MSG_ERR_UNKNOWN_PIN_ID, id, list, idx),
            Error::PinIdSpecifiedTwiceInSameFrame(ref id, ref list, idx1, idx2) =>
                format!("{}: {:?}: {:?}[{}..={}]", MSG_ERR_PIN_ID_SPECIFIED_TWICE_IN_SAME_FRAME, id, list, idx1, idx2),
        })
    }
}
