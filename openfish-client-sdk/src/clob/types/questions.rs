#![allow(
    clippy::module_name_repetitions,
    reason = "Suffix is intentional for clarity"
)]

use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::types::Decimal;

// ─── Question Templates ───

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct ListTemplatesRequest {
    pub category: Option<String>,
    pub search: Option<String>,
    #[builder(default = 50)]
    pub limit: i64,
    #[builder(default = 0)]
    pub offset: i64,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionTemplate {
    pub id: Uuid,
    pub slug: String,
    pub version: i32,
    pub category: Option<String>,
    pub subcategory: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub question_format: String,
    pub parameter_schema: serde_json::Value,
    pub outcome_type: String,
    pub outcome_schema: Option<serde_json::Value>,
    pub resolution_type: Option<String>,
    pub resolution_config: Option<serde_json::Value>,
    pub default_tick_size: Option<Decimal>,
    pub default_fee_bps: Option<i32>,
    pub default_duration_days: Option<i32>,
    pub tags: Option<serde_json::Value>,
    pub data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListTemplatesResponse {
    pub templates: Vec<QuestionTemplate>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetTemplateResponse {
    pub template: QuestionTemplate,
}

// ─── Question Clusters ───

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct ListClustersRequest {
    pub template_id: Option<Uuid>,
    pub search: Option<String>,
    #[builder(default = 50)]
    pub limit: i64,
    #[builder(default = 0)]
    pub offset: i64,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionCluster {
    pub id: Uuid,
    pub template_id: Uuid,
    pub slug: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub cluster_constraints: Option<serde_json::Value>,
    pub tick_size: Option<Decimal>,
    pub fee_bps: Option<i32>,
    pub neg_risk: Option<bool>,
    pub neg_risk_market_id: Option<String>,
    pub market_count: Option<i64>,
    pub bond_required: Option<bool>,
    pub min_bond: Option<Decimal>,
    pub min_fee_rate: Option<Decimal>,
    pub max_fee_rate: Option<Decimal>,
    pub auction_duration_seconds: Option<i32>,
    pub data: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListClustersResponse {
    pub clusters: Vec<QuestionCluster>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GetClusterResponse {
    pub cluster: QuestionCluster,
}

// ─── Create Question ───

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct CreateQuestionRequest {
    pub cluster_id: Uuid,
    pub parameters: serde_json::Value,
    pub outcomes: Option<Vec<String>>,
    pub bond_amount: Option<Decimal>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuestionResponse {
    pub condition_id: String,
    pub question_text: String,
    pub outcomes: Vec<String>,
    pub token_ids: Vec<String>,
    pub status: String,
    pub template_slug: Option<String>,
    pub cluster_slug: Option<String>,
    pub bond_posted: Option<bool>,
}

// ─── Resolve Question ───

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct ResolveQuestionRequest {
    pub condition_id: String,
    pub winning_token_id: String,
    pub resolution_source: Option<String>,
    pub resolution_evidence: Option<serde_json::Value>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolveQuestionResponse {
    pub condition_id: String,
    pub status: String,
    pub message: String,
}

// ─── Related Markets ───

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct RelatedMarketsRequest {
    pub edge_type: Option<String>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LatticeEdge {
    pub id: Uuid,
    pub from_condition_id: String,
    pub to_condition_id: String,
    pub edge_type: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RelatedMarketsResponse {
    pub condition_id: String,
    pub edges: Vec<LatticeEdge>,
    pub count: i64,
}

// ─── Market Fees ───

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeSummary {
    pub total_taker_fees: Decimal,
    pub total_creator_share: Decimal,
    pub total_maker_rebate_share: Decimal,
    pub total_platform_share: Decimal,
    pub trade_count: i64,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketFeesResponse {
    pub condition_id: String,
    pub creator_agent: Option<String>,
    pub creator_fee_rate: Option<Decimal>,
    pub fee_summary: Option<FeeSummary>,
}

// ─── Auctions ───

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct ProposeQuestionRequest {
    pub cluster_id: Uuid,
    pub parameters: serde_json::Value,
    pub proposed_fee_rate: Decimal,
    pub bond_amount: Decimal,
    pub outcomes: Option<Vec<String>>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProposeQuestionResponse {
    pub auction_id: Uuid,
    pub action: String,
    pub status: Option<String>,
    pub end_at: Option<DateTime<Utc>>,
    pub proposed_fee_rate: Option<Decimal>,
    pub bond_amount: Option<Decimal>,
    pub cluster_id: Option<Uuid>,
    pub cluster_slug: Option<String>,
    pub template_slug: Option<String>,
    pub message: Option<String>,
}

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct ListAuctionsRequest {
    pub cluster_id: Option<Uuid>,
    pub status: Option<String>,
    #[builder(default = 50)]
    pub limit: i64,
    #[builder(default = 0)]
    pub offset: i64,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionAuction {
    pub id: Uuid,
    pub cluster_id: Uuid,
    pub parameters: serde_json::Value,
    pub parameters_hash: Option<String>,
    pub status: String,
    pub proposer_agent: Option<String>,
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: Option<DateTime<Utc>>,
    pub winner_agent: Option<String>,
    pub winner_fee_rate: Option<Decimal>,
    pub winner_bond: Option<Decimal>,
    pub condition_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListAuctionsResponse {
    pub auctions: Vec<QuestionAuction>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionBid {
    pub id: Uuid,
    pub auction_id: Uuid,
    pub agent_address: String,
    pub proposed_fee_rate: Decimal,
    pub bond_amount: Decimal,
    pub created_at: DateTime<Utc>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuctionResponse {
    pub auction: QuestionAuction,
    pub bids: Vec<AuctionBid>,
    pub bid_count: i64,
}

#[non_exhaustive]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct AuctionBidRequest {
    pub proposed_fee_rate: Decimal,
    pub bond_amount: Decimal,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionBidResponse {
    pub auction_id: Uuid,
    pub bid_id: Uuid,
    pub proposed_fee_rate: Decimal,
    pub bond_amount: Decimal,
    pub auction_end_at: Option<DateTime<Utc>>,
}

// ─── Bonds ───

#[non_exhaustive]
#[skip_serializing_none]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct ListBondsRequest {
    pub status: Option<String>,
    #[builder(default = 50)]
    pub limit: i64,
    #[builder(default = 0)]
    pub offset: i64,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentBond {
    pub id: Uuid,
    pub agent_address: String,
    pub condition_id: String,
    pub amount: Decimal,
    pub status: String,
    pub slash_reason: Option<String>,
    pub slash_amount: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
    pub slashed_at: Option<DateTime<Utc>>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListBondsResponse {
    pub bonds: Vec<AgentBond>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BondAuditEntry {
    pub id: Uuid,
    pub bond_id: Uuid,
    pub agent_address: String,
    pub condition_id: String,
    pub action: String,
    pub amount: Decimal,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMarketBondResponse {
    pub bond: Option<AgentBond>,
    pub audit_log: Vec<BondAuditEntry>,
}

#[non_exhaustive]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct PostBondRequest {
    pub condition_id: String,
    pub amount: Decimal,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostBondResponse {
    pub bond: AgentBond,
    pub message: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BondRequirementResponse {
    pub cluster_id: Uuid,
    pub bond_required: bool,
    pub min_bond: Decimal,
}

// ─── Disputes ───

#[non_exhaustive]
#[derive(Debug, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(on(String, into))]
pub struct DisputeRequest {
    pub condition_id: String,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisputeResponse {
    pub status: String,
    pub tx_hash: Option<String>,
    pub market_status: Option<String>,
    pub message: String,
}
