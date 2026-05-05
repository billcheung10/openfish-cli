use alloy::primitives::{ChainId, U256};
use bon::Builder;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::types::Address;

/// Request to create deposit addresses for a Openfish wallet.
///
/// # Example
///
/// ```
/// use openfish_client_sdk::types::address;
/// use openfish_client_sdk::bridge::types::DepositRequest;
///
/// let request = DepositRequest::builder()
///     .address(address!("56687bf447db6ffa42ffe2204a05edaa20f55839"))
///     .build();
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Builder)]
pub struct DepositRequest {
    /// The Openfish wallet address to generate deposit addresses for.
    pub address: Address,
}

/// Request to get deposit statuses for a given deposit address.
///
/// ### Note: This doesn't use the alloy Address type, since it supports Solana and Bitcoin addresses.
///
/// # Example
///
/// ```
/// use openfish_client_sdk::bridge::types::StatusRequest;
///
/// let request = StatusRequest::builder().address("0x9cb12Ec30568ab763ae5891ce4b8c5C96CeD72C9").build();
/// ```
#[non_exhaustive]
#[derive(Debug, Clone, Builder)]
#[builder(on(String, into))]
pub struct StatusRequest {
    pub address: String,
}

#[non_exhaustive]
#[serde_as]
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    /// Amount of tokens to send
    #[serde_as(as = "DisplayFromStr")]
    pub from_amount_base_unit: U256,
    /// Source Chain ID
    #[serde_as(as = "DisplayFromStr")]
    pub from_chain_id: ChainId,
    /// Source token address
    pub from_token_address: String,
    /// Address of the recipient
    pub recipient_address: String,
    /// Destination Chain ID
    #[serde_as(as = "DisplayFromStr")]
    pub to_chain_id: ChainId,
    /// Destination token address
    pub to_token_address: String,
}

#[non_exhaustive]
#[serde_as]
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct WithdrawRequest {
    /// Source Openfish wallet address on Polygon
    pub address: Address,
    /// Destination chain ID (e.g., "1" for Ethereum, "8453" for Base, "1151111081099710" for Solana)
    #[serde_as(as = "DisplayFromStr")]
    pub to_chain_id: ChainId,
    /// Destination token contract address
    pub to_token_address: String,
    /// Destination wallet address where funds will be sent
    pub recipient_addr: String,
    /// Amount in base units. Omit or set to "all" to withdraw full available balance minus fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct SwapQuoteRequest {
    /// Openfish wallet address.
    pub address: Address,
    /// Source token. First version supports BNB only.
    pub from_token: String,
    /// Destination FISH token contract address.
    pub to_token_address: String,
    /// BNB amount in wei.
    pub amount_in_wei: String,
    /// Optional slippage in basis points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_bps: Option<u16>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Builder)]
#[builder(on(String, into))]
#[serde(rename_all = "camelCase")]
pub struct SwapExecuteRequest {
    /// Openfish wallet address.
    pub address: Address,
    /// Quote id returned by `swap quote`.
    pub quote_id: String,
}
