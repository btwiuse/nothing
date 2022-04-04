#![feature(derive_default_enum)]

//! nothing
//! =======
//!
//! [![crates.io](https://img.shields.io/crates/v/nothing.svg)](https://crates.io/crates/nothing)
//! [![Documentation](https://docs.rs/nothing/badge.svg)](https://docs.rs/nothing)
//! [![Build Status](https://travis-ci.org/btwiuse/nothing.svg?branch=master)](https://travis-ci.org/btwiuse/nothing)
//!
//! This is my own version of [Option](https://doc.rust-lang.org/stable/std/option/enum.Option.html). Definition:
//!
//! ```
//! pub enum Probably<T> {
//!     Nothing,
//!     Something(T),
//! }
//! ```
//!
//! # Why?
//!
//! The point is that you can use [Probably] as the return type of your main function:
//!
//! ```
//! use nothing::{Probably, Nothing};
//!
//! fn main() -> Probably<()> {
//!     Nothing
//! }
//! ```
//!
//! Exit code is `0` if it is [Something], `1` if [Nothing].
//!
//! See [./examples/main.rs](https://github.com/btwiuse/nothing/blob/master/examples/main.rs)
//!
//! ![Probably::Nothing](https://i.imgur.com/AuDdbOK.png)
//!
//! It's probably nothing.

/// [Probably] is modelled after [Option]:
/// - [Something] is like [Some]
/// - [Nothing] is like [None]
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug, Hash, Default)]
pub enum Probably<T> {
    #[default]
    Nothing,
    Something(T),
}

pub use Probably::{Nothing, Something};

impl<T> std::process::Termination for Probably<T> {
    fn report(self) -> std::process::ExitCode {
        match self {
            Nothing => std::process::ExitCode::FAILURE,
            _ => std::process::ExitCode::SUCCESS,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
