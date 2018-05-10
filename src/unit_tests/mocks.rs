
use Result;
use ten_pin_error::TenPinError;

#[derive(Clone, Default, Debug)]
pub struct MockScoreKeeper {
    pub set_pins: Vec<u8>,
    pub set_pins_times_called: usize,
    pub score: u16,
    pub score_times_called: usize,
}

impl MockScoreKeeper {
    pub fn set_pins(&mut self, pins: u8) -> Result<()> {
        fn validate(pins: u8) -> Result<()> {
            Ok(()).and_then(|| match pins > 10 {
                true => Err(TenPinError::InvalidRollValue)?,
                false => Ok(())
            })
        }
        self.set_pins_times_called += 1;
        self.set_pins.push(pins);
        validate(pins)?;
        self.score += pins;
        Ok(())
    }
}
