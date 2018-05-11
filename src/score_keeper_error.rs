use failure::Fail;

pub trait ScoreKeeperError : Fail + Clone + PartialEq + PartialOrd {}
