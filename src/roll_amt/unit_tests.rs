#![allow(result_unwrap_used)]
use roll_amt::{Error as RollAmountError, RollAmt};
use rspec::{given, run};

#[derive(Clone, Debug)]
struct Env {
    roll_amt_result: Result<RollAmt, RollAmountError>,
}

impl Env {
    fn new() -> Self {
        Self {
            roll_amt_result: RollAmt::new(0),
        }
    }
}
#[test]
fn tests() {
    run(&given("an initialized RollAmt", Env::new(), |ctx| {
        let expected_result = Ok(RollAmt(0_u8));

        ctx.then("the default constructed RollAmt should be 0", move |env| {
            assert!(env.roll_amt_result == expected_result);
        });

        ctx.then("and RollAmt::as_u8 should also be 0_u8", |env| {
            assert!(env.roll_amt_result.clone().unwrap().0 == env.roll_amt_result.clone().unwrap().as_u8());
        });

        ctx.when("initialized to 1_u8", |ctx| {
            ctx.before(|env| {
                env.roll_amt_result = RollAmt::new(1_u8);
            });
            let expected_result = Ok(RollAmt(1_u8));

            ctx.then("the RollAmt should be 1_u8", move |env| {
                assert!(env.roll_amt_result == expected_result);
            });

            ctx.then("and RollAmt::as_u8 should also be 1_u8", |env| {
                assert!(env.roll_amt_result.clone().unwrap().0 == env.roll_amt_result.clone().unwrap().as_u8());
            });
        });

        ctx.when("initialized to 5_i16", |ctx| {
            ctx.before(|env| {
                env.roll_amt_result = RollAmt::new(5_i16);
            });
            let expected_result = Ok(RollAmt(5_u8));

            ctx.then("the RollAmt should be 5_u8", move |env| {
                assert!(env.roll_amt_result == expected_result);
            });
        });

        ctx.when("initialized to 7_u32", |ctx| {
            ctx.before(|env| {
                env.roll_amt_result = RollAmt::new(7_u16);
            });
            let expected_result = Ok(RollAmt(7_u8));

            ctx.then("the RollAmt should be 7_u8", move |env| {
                assert!(env.roll_amt_result == expected_result);
            });
        });

        ctx.when("initialized to 10_u64", |ctx| {
            ctx.before(|env| {
                env.roll_amt_result = RollAmt::new(10_u64);
            });
            let expected_result = Ok(RollAmt(10_u8));

            ctx.then("the RollAmt should be 10_u8", move |env| {
                assert!(env.roll_amt_result == expected_result);
            });

            ctx.then("and RollAmt::as_u8 should also be 10_u8", |env| {
                assert!(env.roll_amt_result.clone().unwrap().0 == env.roll_amt_result.clone().unwrap().as_u8());
            });
        });

        ctx.when("initialized to an invalid RollAmt of 11_i32", |ctx| {
            ctx.before(|env| {
                env.roll_amt_result = RollAmt::new(11_i32);
            });
            let expected_result = Err(RollAmountError::InvalidRollAmt(Some(11_u64)));

            ctx.then("an error should result", move |env| {
                assert!(env.roll_amt_result == expected_result);
            });
        });
    }));
}
