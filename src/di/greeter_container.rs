use greeter::{HelloWorldGreeter, BitWidthProvider, WidthProvider};
use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GreeterContainer {}

impl Container for GreeterContainer {
    fn build() -> Self {
        Self {}
    }
}

impl GreeterContainer {
    pub fn resolve_greeter(&self) -> HelloWorldGreeter<BitWidthProvider> {
        HelloWorldGreeter::new(BitWidthProvider::new())
    }
}