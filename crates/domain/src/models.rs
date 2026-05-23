use std::fmt::{self, Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaAddressError {
    #[error("invalid base58 encoding")]
    InvalidDecode,
    #[error("address length must be 32 bytes")]
    InvalidLen,
}

pub struct SolanaAddress(String);

impl SolanaAddress {
    /// Parses a base58 encoded string into a `SolanaAddress`.
    ///
    /// # Errors
    ///
    /// Returns `SolanaAddressError::InvalidDecode` if the string is not valid base58.
    /// Returns `SolanaAddressError::InvalidLen` if the decoded bytes are not exactly 32 bytes.
    pub fn parse(raw_address: &str) -> Result<Self, SolanaAddressError> {
        let mut output = [0; 32];

        let written_bytes = bs58::decode(raw_address)
            .onto(&mut output)
            .map_err(|_| SolanaAddressError::InvalidDecode)?;

        if written_bytes != 32 {
            return Err(SolanaAddressError::InvalidLen);
        }

        Ok(Self(raw_address.to_string()))
    }
}

impl Display for SolanaAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Tests to cover:
// - Empty string
// - Invalid address (e.g., "acparosiesrtieon2313133213")
// - Valid address
// - Too many and too few characters
// - Non-ASCII characters
// - Invalid Base58 characters
// - Valid Base58 encoding, but invalid size (e.g., 31 or 33 bytes)

#[derive(Error, Debug)]
pub enum LamportsError {
    #[error("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[error("Calculated balance cannot be negative")]
    NegativeBalance,
}

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
