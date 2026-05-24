use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SolanaAddressError {
    #[error("invalid base58 encoding")]
    InvalidDecode,
    #[error("address length must be 32 bytes")]
    InvalidLen,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum LamportsError {
    #[error("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[error("Calculated balance cannot be negative")]
    NegativeBalance,
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum SignatureError {
    #[error("invalid base58 encoding")]
    InvalidDecode,
    #[error("signature length must be 64 bytes")]
    InvalidLen,
}
