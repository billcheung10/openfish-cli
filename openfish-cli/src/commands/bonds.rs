use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use openfish_client_sdk::clob;
use openfish_client_sdk::clob::types::questions::{ListBondsRequest, PostBondRequest};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::auth;
use crate::output::OutputFormat;
use crate::output::bonds::{
    print_bond_requirement, print_bonds, print_market_bond, print_post_bond,
};

#[derive(Args)]
pub struct BondsArgs {
    #[command(subcommand)]
    pub command: BondsCommand,
}

#[derive(Subcommand)]
pub enum BondsCommand {
    /// List bonds for an agent address
    List {
        /// Agent wallet address
        address: String,

        /// Filter by status (ACTIVE, RELEASED, SLASHED)
        #[arg(long)]
        status: Option<String>,

        /// Max results
        #[arg(long, default_value = "50")]
        limit: i64,

        /// Pagination offset
        #[arg(long, default_value = "0")]
        offset: i64,
    },

    /// Get bond for a market
    Get {
        /// Condition ID (0x-prefixed)
        condition_id: String,
    },

    /// Post a bond for an existing market
    Post {
        /// Condition ID (0x-prefixed)
        #[arg(long)]
        condition_id: String,

        /// Bond amount in FISH
        #[arg(long)]
        amount: String,
    },

    /// Get bond requirement for a cluster
    Requirement {
        /// Cluster ID (UUID)
        cluster_id: String,
    },
}

pub async fn execute(
    args: BondsArgs,
    output: OutputFormat,
    private_key: Option<&str>,
    signature_type: Option<&str>,
) -> Result<()> {
    let output = &output;

    match args.command {
        BondsCommand::List {
            address,
            status,
            limit,
            offset,
        } => {
            let client = unauthenticated_client()?;
            let req = ListBondsRequest::builder()
                .maybe_status(status)
                .limit(limit)
                .offset(offset)
                .build();
            let response = client.agent_bonds(&address, &req).await?;
            print_bonds(&response.bonds, output)?;
        }

        BondsCommand::Get { condition_id } => {
            let client = unauthenticated_client()?;
            let response = client.market_bond(&condition_id).await?;
            print_market_bond(&response, output)?;
        }

        BondsCommand::Requirement { cluster_id } => {
            let client = unauthenticated_client()?;
            let cid: Uuid = cluster_id.parse().context("Invalid UUID for cluster-id")?;
            let response = client.bond_requirement(&cid).await?;
            print_bond_requirement(&response, output)?;
        }

        BondsCommand::Post {
            condition_id,
            amount,
        } => {
            let client = auth::authenticated_clob_client(private_key, signature_type).await?;
            let amt: Decimal = amount.parse().context("Invalid decimal for --amount")?;
            let req = PostBondRequest::builder()
                .condition_id(condition_id)
                .amount(amt)
                .build();
            let response = client.post_bond(&req).await?;
            print_post_bond(&response, output)?;
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
