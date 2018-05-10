pub const MSG_ERR_NONE_ERROR: &str = "Unexpected Option::None encountered";
pub const MSG_ERR_INVALID_UTF8_ARG: &str = "Argument containing invalid UTF-8 detected";
pub const MSG_ERR_INTERNAL_SCORE_KEEPER: &str = "Internal error: Score keeper dependency reported an error";
// TenPinError msgs
pub mod ten_pin {
    pub const MSG_ERR_INVALID_ROLL_VALUE: &str = "Invalid roll value received (A roll must be a value from 0 to 10 inclusive)";
}
