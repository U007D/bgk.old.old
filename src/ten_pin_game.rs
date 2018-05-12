use game::Game;

#[derive(Clone, Default, Debug)]
pub struct TenPinGame {}

impl Game for TenPinGame {
    type Score = u16;

    //pub fn rolls(&self, pins: &[PinList]) {}

    fn score(&self) -> Self::Score {
        0
    }
}
