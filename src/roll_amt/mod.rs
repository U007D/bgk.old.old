mod error;

use num::cast::ToPrimitive;
pub use self::error::Error;
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct RollAmt(u8);

impl RollAmt {
    pub fn new<T: ToPrimitive>(val: T) -> ::std::result::Result<Self, Error> {
        #[allow(absurd_extreme_comparisons)]
        match val.to_u8() {
            Some(v) if v >= 0 && v <= 10 => Ok(RollAmt(v)),
            _ => Err(Error::InvalidRollAmt(val.to_u64()))
        }
    }
}
