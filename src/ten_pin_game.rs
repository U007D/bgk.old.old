use game::Game;

#[derive(Clone, Default, Debug)]
pub struct TenPinGame {}

impl Game for TenPinGame {
    type Score = u16;

    fn score(&self, rolls: &[PinList]) -> Self::Score {
        0
    }
}
