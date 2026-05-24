use crate::error::SolanaAddressError;

use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
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

        let written_bytes =
            bs58::decode(raw_address)
                .onto(&mut output)
                .map_err(|err| match err {
                    bs58::decode::Error::BufferTooSmall => SolanaAddressError::InvalidLen,
                    _ => SolanaAddressError::InvalidDecode,
                })?;

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

impl AsRef<str> for SolanaAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    #![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]

    use crate::address::SolanaAddress;
    use crate::error::SolanaAddressError;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_successfully_parse_when_address_is_valid() {
        // Valid address provided
        let raw_address = "4FfpTjYCH9rg8MRFdv3yoL3uJ3SnzKg8p7z2wXpXTBu8";
        let address = SolanaAddress::parse(raw_address).unwrap();

        assert_eq!(address.to_string(), raw_address);
    }

    #[test]
    fn should_return_error_when_address_is_empty() {
        let result = SolanaAddress::parse("");
        // Decoding an empty string yields 0 bytes, which triggers InvalidLen
        assert_eq!(result, Err(SolanaAddressError::InvalidLen));
    }

    #[test]
    fn should_return_error_when_address_is_invalid_base58() {
        let result = SolanaAddress::parse("acparosiesrtieon2313133213");
        // This string contains valid base58 characters but its length is incorrect.
        assert_eq!(result, Err(SolanaAddressError::InvalidLen));
    }

    #[test]
    fn should_return_error_when_address_has_invalid_base58_characters() {
        // '0' is not a valid base58 character
        let result = SolanaAddress::parse("4FfpTjYCH9rg8MRFdv3yoL3uJ3SnzKg8p7z2wXpXTBu0");
        assert_eq!(result, Err(SolanaAddressError::InvalidDecode));
    }

    #[test]
    fn should_return_error_when_address_has_non_ascii_characters() {
        // 'ü' is non-ascii
        let result = SolanaAddress::parse("4FfpTjYCH9rg8MRFdv3yoL3uJ3SnzKg8p7z2wXpXTBü8");
        assert_eq!(result, Err(SolanaAddressError::InvalidDecode));
    }

    #[test]
    fn should_return_error_when_decoded_bytes_is_too_short() {
        // 31 '1's = 31 bytes of zeros
        let result = SolanaAddress::parse("1111111111111111111111111111111");
        assert_eq!(result, Err(SolanaAddressError::InvalidLen));
    }

    #[test]
    fn should_return_error_when_decoded_bytes_is_too_long() {
        // 33 '1's = 33 bytes of zeros
        let result = SolanaAddress::parse("111111111111111111111111111111111");
        // Because the buffer `output` is 32 bytes, bs58::decode returns a BufferTooSmall error
        // which the current implementation maps to InvalidDecode.
        assert_eq!(result, Err(SolanaAddressError::InvalidLen));
    }
}
