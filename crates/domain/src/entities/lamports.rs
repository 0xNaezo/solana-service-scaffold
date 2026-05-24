use crate::error::LamportsError;

use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Lamports(u64);

impl Lamports {
    #[must_use]
    pub const fn new(raw_lamport: u64) -> Self {
        Self(raw_lamport)
    }

    /// Adds two `Lamports` together.
    ///
    /// # Errors
    ///
    /// Returns `LamportsError::ArithmeticOverflow` if the addition overflows.
    pub const fn checked_add(self, other: &Self) -> Result<Self, LamportsError> {
        match self.0.checked_add(other.0) {
            Some(lamports) => Ok(Self(lamports)),

            None => Err(LamportsError::ArithmeticOverflow),
        }
    }

    /// Subtracts two `Lamports`.
    ///
    /// # Errors
    ///
    /// Returns `LamportsError::NegativeBalance` if the subtraction underflows.
    pub const fn checked_sub(self, other: &Self) -> Result<Self, LamportsError> {
        match self.0.checked_sub(other.0) {
            Some(lamports) => Ok(Self(lamports)),

            None => Err(LamportsError::NegativeBalance),
        }
    }

    #[must_use]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}

impl Display for Lamports {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Lamports> for u64 {
    fn from(lamports: Lamports) -> Self {
        lamports.0
    }
}

#[cfg(test)]
mod tests {
    #![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]

    use crate::entities::lamports::Lamports;
    use crate::error::LamportsError;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_return_error_when_addition_overflows() {
        // Adding 10 to u64::MAX causes an arithmetic overflow
        let first = Lamports::new(u64::MAX);
        let second = Lamports::new(10);
        let result = first.checked_add(&second);
        assert_eq!(result, Err(LamportsError::ArithmeticOverflow));
    }

    #[test]
    fn should_return_negative_balance_error_when_subtraction_underflows() {
        // Subtracting 9999 from 10 causes subtraction underflow
        let first = Lamports::new(10);
        let second = Lamports::new(9999);
        let result = first.checked_sub(&second);
        assert_eq!(result, Err(LamportsError::NegativeBalance));
    }

    #[test]
    fn should_return_original_value_when_adding_or_subtracting_zero() {
        // Adding or subtracting zero to/from any value returns the original value
        assert_eq!(
            Lamports::new(0).checked_add(&Lamports::new(0)).unwrap(),
            Lamports::new(0)
        );
        assert_eq!(
            Lamports::new(1000).checked_add(&Lamports::new(0)).unwrap(),
            Lamports::new(1000)
        );
        assert_eq!(
            Lamports::new(0).checked_sub(&Lamports::new(0)).unwrap(),
            Lamports::new(0)
        );
        assert_eq!(
            Lamports::new(1000).checked_sub(&Lamports::new(0)).unwrap(),
            Lamports::new(1000)
        );
    }

    #[test]
    fn should_successfully_add_two_values() {
        // Successfully adding two values together without overflow
        let first = Lamports::new(10);
        let second = Lamports::new(20);

        let result = first.checked_add(&second).unwrap();

        assert_eq!(result, Lamports::new(30));
    }

    #[test]
    fn should_successfully_subtract_two_values() {
        // Successfully subtracting two values without underflow
        let first = Lamports::new(50);
        let second = Lamports::new(20);

        let result = first.checked_sub(&second).unwrap();

        assert_eq!(result, Lamports::new(30));
    }
}
