use crate::error::LamportsError;

use std::fmt::{self, Display, Formatter};

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
}

impl Display for Lamports {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Tests to cover:
// - Overflow during addition
// - Underflow/negative balance during subtraction
// - Addition of zeros
