use crate::error::SignatureError;

use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Signature(String);

impl Signature {
    /// Parses a base58 encoded string into a `Signature`.
    ///
    /// # Errors
    ///
    /// Returns `SignatureError::InvalidDecode` if the string is not valid base58.
    /// Returns `SignatureError::InvalidLen` if the decoded bytes are not exactly 64 bytes.
    pub fn parse(raw_address: &str) -> Result<Self, SignatureError> {
        let mut output = [0; 64];

        let written_bytes =
            bs58::decode(raw_address)
                .onto(&mut output)
                .map_err(|err| match err {
                    bs58::decode::Error::BufferTooSmall => SignatureError::InvalidLen,
                    _ => SignatureError::InvalidDecode,
                })?;

        if written_bytes != 64 {
            return Err(SignatureError::InvalidLen);
        }

        Ok(Self(raw_address.to_string()))
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Signature {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    #![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]

    use crate::error::SignatureError;
    use crate::signature::Signature;

    use pretty_assertions::assert_eq;

    #[test]
    fn should_successfully_parse_when_signature_is_valid() {
        let valid_signature = "5Fmat5M861FpdJApmx9RJQtC2SxB4XpNJHoLtFZ5EYfmrmSqYtSdWHzWi4WUuBQAtXtaLqBrFYT9yU1B1ZS4ey3R";
        let result = Signature::parse(valid_signature).unwrap();
        // Valid signature provided
        assert_eq!(result.to_string(), valid_signature);
    }

    #[test]
    fn should_return_error_when_signature_is_empty() {
        let result = Signature::parse("");
        // Decoding an empty string yields 0 bytes, which triggers InvalidLen
        assert_eq!(result, Err(SignatureError::InvalidLen));
    }

    #[test]
    fn should_return_error_when_signature_is_invalid() {
        let result = Signature::parse("a+par0sie0rtieIOO2313=13/3213");
        // This string contains invalid characters (e.g., '+', '=', '/') and invalid len
        assert_eq!(result, Err(SignatureError::InvalidDecode));
    }

    #[test]
    fn should_return_error_when_signature_has_invalid_base58_characters() {
        // '0' is not a valid base58 character
        let invalid_signature = format!(
            "{}0",
            "5V8Mmvzp7vuG9Agy97dZJYBcxZuAbbFt1Juf2u3gqYT62iYtNMQwDcmEBP4HBojRRcZUevPneTaYZge5RyYR4zS"
        );
        let result = Signature::parse(&invalid_signature);
        assert_eq!(result, Err(SignatureError::InvalidDecode));
    }

    #[test]
    fn should_return_error_when_signature_has_non_ascii_characters() {
        // 'é' is non-ascii
        let invalid_signature = format!(
            "{}é",
            "34wE3aTShT44mEPCeCv81JrKFGyvrTy6d8dMcrhWWszxf6khNGSozqVBhD2pnuGp4WtwYiqxaecPwBCp6Md8r3f"
        );
        let result = Signature::parse(&invalid_signature);
        assert_eq!(result, Err(SignatureError::InvalidDecode));
    }

    #[test]
    fn should_return_error_when_signature_has_too_few_characters() {
        // 63 '1's = 63 bytes of zeros
        let invalid_signature = "1".repeat(63);
        let result = Signature::parse(&invalid_signature);
        assert_eq!(result, Err(SignatureError::InvalidLen));
    }

    #[test]
    fn should_return_error_when_signature_has_too_many_characters() {
        // 65 '1's = 65 bytes of zeros
        let invalid_signature = "1".repeat(65);
        // Because the buffer `output` is 64 bytes, bs58::decode returns a BufferTooSmall error
        // which the current implementation maps to InvalidLen.
        let result = Signature::parse(&invalid_signature);
        assert_eq!(result, Err(SignatureError::InvalidLen));
    }
}
