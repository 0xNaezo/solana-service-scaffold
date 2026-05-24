use async_trait::async_trait;

use crate::{
    entities::{
        address::SolanaAddress, dto::RawTransactionData, lamports::Lamports, signature::Signature,
    },
    error::ProviderError,
};

#[async_trait]
pub trait SolanaProvider {
    async fn get_balance(address: SolanaAddress) -> Result<Lamports, ProviderError>;

    async fn get_signatures_for_address(
        address: SolanaAddress,
    ) -> Result<Vec<Signature>, ProviderError>;

    async fn get_transaction(address: SolanaAddress) -> Result<RawTransactionData, ProviderError>;
}
