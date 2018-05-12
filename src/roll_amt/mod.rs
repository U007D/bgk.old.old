mod error;
#[cfg(test)]
mod unit_tests;

use num::cast::ToPrimitive;
pub use self::error::Error;

type Result<T> = ::std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RollAmt(u8);

impl RollAmt {
    pub fn new<T: ToPrimitive + Copy>(val: T) -> Result<Self> {
        #[allow(absurd_extreme_comparisons)]    // retain v >= 0 to protect against changes to signed types
        #[allow(unused_comparisons)]            // retain v >= 0 to protect against changes to signed types
        match val.to_u8() {
            Some(v) if v >= 0 && v <= 10 => Ok(RollAmt(v)),
            _ => Err(Error::InvalidRollAmt(val.to_u64()))
        }
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }
}
