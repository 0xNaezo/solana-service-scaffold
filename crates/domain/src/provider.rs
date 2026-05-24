use async_trait::async_trait;

use crate::{
    address::SolanaAddress, dto::RawTransactionData, error::ProviderError, lamports::Lamports,
    signature::Signature,
};

#[async_trait]
pub trait SolanaProvider {
    async fn get_balance(address: SolanaAddress) -> Result<Lamports, ProviderError>;

    async fn get_signatures_for_address(
        address: SolanaAddress,
    ) -> Result<Vec<Signature>, ProviderError>;

    async fn get_transaction(address: SolanaAddress) -> Result<RawTransactionData, ProviderError>;
}
