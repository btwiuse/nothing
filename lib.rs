//! nothing
//! =======
//!
//! [![crates.io](https://img.shields.io/crates/v/nothing.svg)](https://crates.io/crates/nothing)
//! [![Documentation](https://docs.rs/nothing/badge.svg)](https://docs.rs/nothing)
//! [![Build Status](https://travis-ci.org/btwiuse/nothing.svg?branch=master)](https://travis-ci.org/btwiuse/nothing)
//!
//! nothing::[Probably] is a better [Option].
//!
//! ```no_run
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
//! ```no_run
//! use nothing::{Probably, Nothing};
//!
//! fn main() -> Probably<()> {
//!     Nothing
//! }
//! ```
//!
//! Exit code is `0` if it is [Something], `1` if [Nothing].
//!
//! You can even use the `?` operator the way you would with [Option] and [Result]. See [./examples/main.rs](https://github.com/btwiuse/nothing/blob/master/examples/main.rs)
//!
//! ![Probably::Nothing](https://camo.githubusercontent.com/8bfa566db90d366cb0dd026267f78a7dfca0c3193cb84172b90d05b594b7062c/68747470733a2f2f692e696d6775722e636f6d2f41754464624f4b2e706e67)
//!
//! Probably nothing.
#![feature(derive_default_enum)]
#![feature(try_trait_v2)]

/// [Probably] is a better [Option]:
/// - [Something] is like [Some]
/// - [Nothing] is like [None]
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug, Hash, Default)]
pub enum Probably<T> {
    /// No value.
    #[default]
    Nothing,
    /// Some value of type `T`.
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

impl<T> std::ops::FromResidual for Probably<T> {
    fn from_residual(residual: Probably<std::convert::Infallible>) -> Self {
        match residual {
            Nothing => Nothing,
            Something(_) => todo!(),
        }
    }
}

impl<T> std::ops::Try for Probably<T> {
    type Output = T;
    type Residual = Probably<std::convert::Infallible>;

    fn from_output(output: Self::Output) -> Self {
        Something(output)
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Something(v) => std::ops::ControlFlow::Continue(v),
            Nothing => std::ops::ControlFlow::Break(Nothing),
        }
    }
}

impl<T> From<T> for Probably<T> {
    fn from(x: T) -> Self {
        Something(x)
    }
}

impl<T> From<Option<T>> for Probably<T> {
    fn from(x: Option<T>) -> Self {
        match x {
            Some(v) => Something(v),
            None => Nothing,
        }
    }
}

impl<T> Into<Option<T>> for Probably<T> {
    fn into(self) -> Option<T> {
        match self {
            Something(x) => Some(x),
            Nothing => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn from_option_works() {
        let some = Some(42u8);
        let sth: Probably<u8> = some.into();
        assert_eq!(sth, Something(42u8));
    }

    #[test]
    fn from_unit_works() {
        let unit = ();
        let sth: Probably<()> = unit.into();
        assert_eq!(sth, Something(()));
    }

    #[test]
    fn from_some_works() {
        let unit = Some(());
        let sth: Probably<()> = unit.into();
        assert_eq!(sth, Something(()));
    }
}
