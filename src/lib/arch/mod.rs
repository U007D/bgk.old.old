use super::*;

#[cfg(test)] mod unit_tests;

use galvanic_mock::{mockable, use_mocks};

#[mockable]
pub trait Info {
    /// # Remarks
    /// Returns the native width of the current architecture, in bits
    fn width(&self) -> usize;
}

pub struct Arch;

impl Arch {
    pub fn new() -> Self { Self {} }
}

impl Info for Arch {
    fn width(&self) -> usize {
        0_usize.count_zeros() as usize
    }
}
