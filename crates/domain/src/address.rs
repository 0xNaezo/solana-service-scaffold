use crate::error::SolanaAddressError;

use std::fmt::{self, Display, Formatter};

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
