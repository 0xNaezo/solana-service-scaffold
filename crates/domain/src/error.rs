use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaAddressError {
    #[error("invalid base58 encoding")]
    InvalidDecode,
    #[error("address length must be 32 bytes")]
    InvalidLen,
}

#[derive(Error, Debug)]
pub enum LamportsError {
    #[error("Arithmetic overflow occurred")]
    ArithmeticOverflow,
    #[error("Calculated balance cannot be negative")]
    NegativeBalance,
}
