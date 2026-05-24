use crate::entities::{address::SolanaAddress, lamports::Lamports, signature::Signature};

#[derive(Debug, Clone)]
pub struct RawTransactionData {
    pub slot: u64,
    pub signature: Signature,
    pub fee: Lamports,
    pub instructions: Vec<RawInstruction>,
}

#[derive(Debug, Clone)]
pub struct RawInstruction {
    pub program_id: SolanaAddress,
    pub data: Vec<u8>,
}

impl RawTransactionData {
    #[must_use]
    pub const fn new(
        slot: u64,
        signature: Signature,
        fee: Lamports,
        instructions: Vec<RawInstruction>,
    ) -> Self {
        Self {
            slot,
            signature,
            fee,
            instructions,
        }
    }
}

impl RawInstruction {
    #[must_use]
    pub const fn new(program_id: SolanaAddress, data: Vec<u8>) -> Self {
        Self { program_id, data }
    }
}
