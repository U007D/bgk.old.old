#![cfg_attr(feature="clippy", feature(plugin))] //nightly rustc required by `clippy`
#![cfg_attr(feature="clippy", plugin(clippy))]
#![feature(proc_macro, const_atomic_bool_new, galvanic_mock_integration)] //req'd by galvanic-mock, galvanic-test + galvanic-mock
#![allow(match_bool, redundant_closure /*galvanic*/, unnecessary_mut_passed /*galvanic*/)] //disable false positives
#![warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss, empty_enum, enum_glob_use,
        fallible_impl_from, filter_map, if_not_else, int_plus_one, invalid_upcast_comparisons, maybe_infinite_iter,
        mem_forget, missing_debug_implementations, mut_mut, mutex_integer, nonminimal_bool, option_map_unwrap_or,
        option_map_unwrap_or_else, option_map_unwrap_or_else, option_unwrap_used, /*print_stdout,*/
        pub_enum_variant_names, range_plus_one, result_map_unwrap_or_else, result_unwrap_used, /*trivial_casts, //galvanic*/
        non_camel_case_types, stutter, trivial_numeric_casts, unicode_not_nfc, unseparated_literal_suffix,
        /*use_debug,*/ use_self, used_underscore_binding, unused_import_braces, /*unused_qualifications, //galvanic*/
        wrong_pub_self_convention)]
#![deny(overflowing_literals, unused_must_use)]

#[macro_use] pub extern crate failure;
extern crate polish;

#[cfg(test)] mod unit_tests;
pub mod consts;
mod error;

pub use error::Error;
#[allow(unused_imports)] use consts::*;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub struct App {
    args: Vec<String>,
    arch_width: usize,
}

impl App {
    /// # Returns
    /// Application instance upon successful initialization.
    pub fn new(args: Vec<String>, arch_width: usize) -> Self {
        Self {
            args,
            arch_width,
        }
    }

    /// # Returns
    /// Result of application execution.
    ///
    /// # Errors
    /// Returns an error in the event that any unhandled errors arise during execution.
    ///
    /// # Remarks
    /// This method is the library's primary entry point.
    pub fn run(&self) -> Result<String> {
        self.greet()
    }

    fn greet(&self) -> Result<String> { Ok(format!("Hello, {}-bit world!", self.arch_width)) }
}
