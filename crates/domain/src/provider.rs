use async_trait::async_trait;

use crate::{
    entities::{
        address::SolanaAddress, dto::RawTransactionData, lamports::Lamports, signature::Signature,
    },
    error::ProviderError,
};

#[async_trait]
pub trait SolanaProvider: Send + Sync {
    async fn get_balance(&self, address: SolanaAddress) -> Result<Lamports, ProviderError>;

    async fn get_signatures_for_address(
        &self,
        address: SolanaAddress,
    ) -> Result<Vec<Signature>, ProviderError>;

    async fn get_transaction(
        &self,
        signature: Signature,
    ) -> Result<RawTransactionData, ProviderError>;
}
