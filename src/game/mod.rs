mod error;

pub use Error;
use num_traits::Num;

pub trait Game : Default {
    type Score: Num;

    fn new() -> Self {
        Self::default()
    }

    fn score(&self, rolls: &[PinList]) -> Self::Score;
}
