use alloy::primitives::U256;
use bon::Builder;
use serde::{Deserialize, Serialize};
use serde_with::{DisplayFromStr, serde_as};

use crate::types::{Address, ChainId, Decimal};

/// Response containing deposit addresses for different blockchain networks.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
pub struct DepositResponse {
    /// Deposit addresses for different blockchain networks.
    pub address: DepositAddresses,
    /// Additional information about supported chains.
    pub note: Option<String>,
}

/// Deposit addresses for different blockchain networks.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
pub struct DepositAddresses {
    /// EVM-compatible deposit address (Ethereum, Polygon, Arbitrum, Base, etc.).
    pub evm: Address,
    /// Solana Virtual Machine deposit address.
    #[serde(default)]
    pub svm: Option<String>,
    /// Bitcoin deposit address.
    #[serde(default)]
    pub btc: Option<String>,
}

/// Response containing all supported assets for deposits.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[serde(rename_all = "camelCase")]
pub struct SupportedAssetsResponse {
    /// List of supported assets with minimum deposit amounts.
    pub supported_assets: Vec<SupportedAsset>,
    /// Additional information about supported chains and assets.
    pub note: Option<String>,
}

/// A supported asset with chain and token information.
#[non_exhaustive]
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct SupportedAsset {
    /// Blockchain chain ID (e.g., 1 for Ethereum mainnet, 137 for Polygon).
    /// Deserialized from JSON string representation (e.g., `"137"`).
    #[serde_as(as = "DisplayFromStr")]
    pub chain_id: ChainId,
    /// Human-readable chain name.
    pub chain_name: String,
    /// Token information.
    pub token: Token,
    /// Minimum deposit amount in USD.
    pub min_checkout_usd: Decimal,
}

/// Token information for a supported asset.
#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
pub struct Token {
    /// Full token name.
    pub name: String,
    /// Token symbol.
    pub symbol: String,
    /// Token contract address.
    pub address: String,
    /// Token decimals.
    pub decimals: u8,
}

/// Transaction status for all deposits associated with a given deposit address.
#[non_exhaustive]
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    #[serde(default)]
    pub deposits: Vec<DepositTransaction>,
    #[serde(default)]
    pub withdrawals: Vec<WithdrawalTransaction>,
    /// List of transactions for the given address
    #[serde(default)]
    pub transactions: Vec<DepositTransaction>,
}

#[non_exhaustive]
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct DepositTransaction {
    /// Source chain ID
    #[serde_as(as = "DisplayFromStr")]
    pub from_chain_id: ChainId,
    /// Source token contract address
    pub from_token_address: String,
    /// Amount in base units (without decimals)
    #[serde_as(as = "DisplayFromStr")]
    pub from_amount_base_unit: U256,
    /// Destination chain ID
    #[serde_as(as = "DisplayFromStr")]
    pub to_chain_id: ChainId,
    /// Destination chain ID
    pub to_token_address: Address,
    /// Current status of the transaction
    pub status: DepositTransactionStatus,
    /// Transaction hash (only available when status is Completed)
    pub tx_hash: Option<String>,
    /// Unix timestamp in milliseconds when transaction was created (missing when status is `DepositDetected`)
    pub created_time_ms: Option<u64>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DepositTransactionStatus {
    DepositDetected,
    Processing,
    OriginTxConfirmed,
    Submitted,
    Completed,
    Failed,
}

#[non_exhaustive]
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalTransaction {
    #[serde_as(as = "DisplayFromStr")]
    pub to_chain_id: ChainId,
    pub to_token_address: String,
    pub recipient_addr: String,
    pub status: String,
    pub tx_hash: Option<String>,
    pub gas_fee_usd: Option<String>,
    pub amount_base_unit: Option<String>,
    pub fee_base_unit: Option<String>,
    pub created_time_ms: Option<u64>,
}

#[non_exhaustive]
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    /// Estimated time to complete the checkout in milliseconds
    pub est_checkout_time_ms: u64,
    /// Breakdown of the estimated fees
    pub est_fee_breakdown: EstimatedFeeBreakdown,
    /// Estimated token amount received in USD
    pub est_input_usd: f64,
    /// Estimated token amount sent in USD
    pub est_output_usd: f64,
    /// Estimated token amount received
    #[serde_as(as = "DisplayFromStr")]
    pub est_to_token_base_unit: U256,
    /// Unique quote id of the request
    pub quote_id: String,
}

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct EstimatedFeeBreakdown {
    /// Label of the app fee
    pub app_fee_label: String,
    /// App fees as a percentage of the total amount sent
    pub app_fee_percent: f64,
    /// App fees in USD
    pub app_fee_usd: f64,
    /// Fill cost percentage of the total amount sent
    pub fill_cost_percent: f64,
    /// Fill cost in USD
    pub fill_cost_usd: f64,
    /// Gas fee in USD
    pub gas_usd: f64,
    /// Maximum potential slippage as a percentage
    pub max_slippage: f64,
    /// Amount after factoring slippage
    pub min_received: f64,
    /// Swap impact as a percentage of the total amount sent
    pub swap_impact: f64,
    /// Swap impact of the transaction in USD
    pub swap_impact_usd: f64,
    /// Total impact as a percentage of the total amount sent
    pub total_impact: f64,
    /// Impact cost of the transaction
    pub total_impact_usd: f64,
}

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
pub struct WithdrawResponse {
    /// Deposit addresses for different blockchain networks
    pub address: WithdrawalAddresses,
    /// Additional information about the deposit addresses
    pub note: String,
}

#[non_exhaustive]
#[serde_as]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct WithdrawPreviewResponse {
    pub address: String,
    #[serde_as(as = "DisplayFromStr")]
    pub to_chain_id: ChainId,
    pub chain_name: String,
    pub token: Token,
    pub recipient_addr: String,
    pub clob_balance: String,
    pub clob_balance_base_unit: String,
    pub amount_base_unit: String,
    pub fee_base_unit: String,
    pub total_debit_base_unit: String,
    pub hot_wallet_balance_base_unit: String,
    pub gas_bnb_wei: String,
    pub fee_source: String,
    pub can_withdraw: bool,
}

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, PartialEq, Builder)]
#[builder(on(String, into))]
pub struct WithdrawalAddresses {
    /// EVM-compatible deposit address (Ethereum, Polygon, Arbitrum, Base, etc.).
    pub evm: Address,
    /// Solana Virtual Machine deposit address.
    #[serde(default)]
    pub svm: Option<String>,
    /// Bitcoin deposit address.
    #[serde(default)]
    pub btc: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct SwapQuoteResponse {
    pub quote_id: String,
    pub address: String,
    pub deposit_address: String,
    pub chain_id: String,
    pub from_token: String,
    pub to_token_address: String,
    pub amount_in_wei: String,
    pub estimated_amount_out_base_unit: String,
    pub min_amount_out_base_unit: String,
    pub slippage_bps: u16,
    pub price_impact_bps: Option<u64>,
    pub route: Vec<String>,
    pub deposit_bnb_balance_wei: String,
    pub required_bnb_wei: String,
    pub has_enough_bnb: bool,
    pub expires_at: String,
}

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct SwapExecuteResponse {
    pub swap_id: String,
    pub quote_id: String,
    pub address: String,
    pub deposit_address: String,
    pub status: String,
    pub swap_tx_hash: String,
    pub sweep_tx_hash: Option<String>,
    pub amount_in_wei: String,
    pub min_amount_out_base_unit: String,
    pub fish_received_base_unit: Option<String>,
    pub fish_credited_base_unit: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct SwapStatusResponse {
    pub swap_id: String,
    pub quote_id: String,
    pub address: String,
    pub deposit_address: String,
    pub status: String,
    pub amount_in_wei: String,
    pub min_amount_out_base_unit: String,
    pub fish_received_base_unit: Option<String>,
    pub fish_credited_base_unit: Option<String>,
    pub swap_tx_hash: Option<String>,
    pub sweep_tx_hash: Option<String>,
    pub error: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
