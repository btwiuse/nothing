#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

#![feature(derive_default_enum)]
#![feature(try_trait_v2)]

/// [Probably] is a better [Option]:
/// - [Something] is like [Some]
/// - [Nothing] is like [None]
#[derive(is_macro::Is, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug, Hash, Default)]
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

impl<T> Probably<Probably<T>> {
    /// Converts from `Probably<Probably<T>>` to `Probably<T>`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// let x: Probably<Probably<u32>> = Something(Something(6));
    /// assert_eq!(Something(6), x.flatten());
    ///
    /// let x: Probably<Probably<u32>> = Something(Nothing);
    /// assert_eq!(Nothing, x.flatten());
    ///
    /// let x: Probably<Probably<u32>> = Nothing;
    /// assert_eq!(Nothing, x.flatten());
    /// ```
    ///
    ///
    /// Flattening only removes one level of nesting at a time:
    ///
    /// ```
    /// let x: Probably<Probably<Probably<u32>>> = Something(Something(Something(6)));
    /// assert_eq!(Something(Something(6)), x.flatten());
    /// assert_eq!(Something(6), x.flatten().flatten());
    /// ```
    pub fn flatten(self) -> Probably<T> {
        match self {
            Something(inner) => inner,
            Nothing => Nothing,
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
