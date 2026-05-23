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
