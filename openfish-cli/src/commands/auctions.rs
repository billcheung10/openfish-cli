use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use openfish_client_sdk::clob;
use openfish_client_sdk::clob::types::questions::{
    AuctionBidRequest, ListAuctionsRequest, ProposeQuestionRequest,
};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::auth;
use crate::output::OutputFormat;
use crate::output::auctions::{
    print_auction, print_auctions, print_bid_result, print_propose_result,
};

#[derive(Args)]
pub struct AuctionsArgs {
    #[command(subcommand)]
    pub command: AuctionsCommand,
}

#[derive(Subcommand)]
pub enum AuctionsCommand {
    /// List fee rate auctions
    List {
        /// Filter by cluster ID (UUID)
        #[arg(long)]
        cluster_id: Option<String>,

        /// Filter by status (BIDDING, CLOSED, AWARDED, EXPIRED, CANCELED)
        #[arg(long)]
        status: Option<String>,

        /// Max results
        #[arg(long, default_value = "50")]
        limit: i64,

        /// Pagination offset
        #[arg(long, default_value = "0")]
        offset: i64,
    },

    /// Get an auction with its bids
    Get {
        /// Auction ID (UUID)
        id: String,
    },

    /// Propose a question and open a fee rate auction
    Propose {
        /// Cluster ID (UUID)
        #[arg(long)]
        cluster_id: String,

        /// Parameters as JSON string
        #[arg(long)]
        parameters: String,

        /// Proposed fee rate (e.g. 0.025 for 2.5%)
        #[arg(long)]
        proposed_fee_rate: String,

        /// Bond amount in FISH
        #[arg(long)]
        bond_amount: String,

        /// Outcomes (comma-separated, e.g. "Yes,No")
        #[arg(long)]
        outcomes: Option<String>,
    },

    /// Submit a competing bid on an auction
    Bid {
        /// Auction ID (UUID)
        #[arg(long)]
        auction_id: String,

        /// Proposed fee rate (e.g. 0.020 for 2%)
        #[arg(long)]
        proposed_fee_rate: String,

        /// Bond amount in FISH
        #[arg(long)]
        bond_amount: String,
    },
}

pub async fn execute(
    args: AuctionsArgs,
    output: OutputFormat,
    private_key: Option<&str>,
    signature_type: Option<&str>,
) -> Result<()> {
    let output = &output;

    match args.command {
        AuctionsCommand::List {
            cluster_id,
            status,
            limit,
            offset,
        } => {
            let client = unauthenticated_client()?;
            let cid = cluster_id
                .map(|s| s.parse::<Uuid>())
                .transpose()
                .context("Invalid UUID for --cluster-id")?;
            let req = ListAuctionsRequest::builder()
                .maybe_cluster_id(cid)
                .maybe_status(status)
                .limit(limit)
                .offset(offset)
                .build();
            let response = client.list_auctions(&req).await?;
            print_auctions(&response.auctions, output)?;
        }

        AuctionsCommand::Get { id } => {
            let client = unauthenticated_client()?;
            let uuid: Uuid = id.parse().context("Invalid UUID for auction ID")?;
            let response = client.get_auction(&uuid).await?;
            print_auction(&response, output)?;
        }

        AuctionsCommand::Propose {
            cluster_id,
            parameters,
            proposed_fee_rate,
            bond_amount,
            outcomes,
        } => {
            let client = auth::authenticated_clob_client(private_key, signature_type).await?;
            let cid: Uuid = cluster_id
                .parse()
                .context("Invalid UUID for --cluster-id")?;
            let params: serde_json::Value =
                serde_json::from_str(&parameters).context("Invalid JSON for --parameters")?;
            let fee: Decimal = proposed_fee_rate
                .parse()
                .context("Invalid decimal for --proposed-fee-rate")?;
            let bond: Decimal = bond_amount
                .parse()
                .context("Invalid decimal for --bond-amount")?;
            let outcomes = outcomes.map(|s| s.split(',').map(|o| o.trim().to_string()).collect());

            let req = ProposeQuestionRequest::builder()
                .cluster_id(cid)
                .parameters(params)
                .proposed_fee_rate(fee)
                .bond_amount(bond)
                .maybe_outcomes(outcomes)
                .build();
            let response = client.propose_question(&req).await?;
            print_propose_result(&response, output)?;
        }

        AuctionsCommand::Bid {
            auction_id,
            proposed_fee_rate,
            bond_amount,
        } => {
            let client = auth::authenticated_clob_client(private_key, signature_type).await?;
            let aid: Uuid = auction_id
                .parse()
                .context("Invalid UUID for --auction-id")?;
            let fee: Decimal = proposed_fee_rate
                .parse()
                .context("Invalid decimal for --proposed-fee-rate")?;
            let bond: Decimal = bond_amount
                .parse()
                .context("Invalid decimal for --bond-amount")?;

            let req = AuctionBidRequest::builder()
                .proposed_fee_rate(fee)
                .bond_amount(bond)
                .build();
            let response = client.auction_bid(&aid, &req).await?;
            print_bid_result(&response, output)?;
        }
    }

    Ok(())
}

fn unauthenticated_client() -> Result<clob::Client> {
    match std::env::var("OPENFISH_CLOB_HOST") {
        Ok(h) if !h.is_empty() => {
            clob::Client::new(&h, clob::Config::default()).context("Failed to create CLOB client")
        }
        _ => Ok(clob::Client::default()),
    }
}
