use anyhow::Result;
use clap::{Args, Subcommand};
use openfish_client_sdk::types::Address;

use crate::auth;
use crate::output::OutputFormat;

#[derive(Args)]
pub struct ApproveArgs {
    #[command(subcommand)]
    pub command: ApproveCommand,
}

#[derive(Subcommand)]
pub enum ApproveCommand {
    /// Show whether on-chain approvals are required for the current deployment
    Check {
        /// Wallet address to check (defaults to configured wallet)
        address: Option<Address>,
    },
    /// Compatibility command. Current FISH ledger trading does not require approvals.
    Set,
}

pub async fn execute(
    args: ApproveArgs,
    output: OutputFormat,
    private_key: Option<&str>,
) -> Result<()> {
    match args.command {
        ApproveCommand::Check { address } => check(address, private_key, output).await,
        ApproveCommand::Set => set(private_key, output).await,
    }
}

async fn check(
    address_arg: Option<Address>,
    private_key: Option<&str>,
    output: OutputFormat,
) -> Result<()> {
    let owner = if let Some(addr) = address_arg {
        Some(addr)
    } else {
        let signer = auth::resolve_signer(private_key)?;
        Some(openfish_client_sdk::auth::Signer::address(&signer))
    };

    print_not_required(output, owner)
}

async fn set(private_key: Option<&str>, output: OutputFormat) -> Result<()> {
    let owner = auth::resolve_signer(private_key)
        .ok()
        .map(|signer| openfish_client_sdk::auth::Signer::address(&signer));

    print_not_required(output, owner)
}

fn print_not_required(output: OutputFormat, owner: Option<Address>) -> Result<()> {
    match output {
        OutputFormat::Table => {
            if let Some(owner) = owner {
                println!("Wallet: {owner}");
            }
            println!("No on-chain approvals are required for current Openfish trading.");
            println!("Openfish uses BSC FISH deposits into an off-chain CLOB ledger.");
            println!("Fund your ledger with: openfish bridge deposit <wallet-address>");
        }
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "required": false,
                    "chain": "BSC",
                    "chain_id": 56,
                    "asset": "FISH",
                    "settlement": "offchain-ledger",
                    "wallet": owner.map(|a| a.to_string()),
                    "message": "No on-chain approvals are required for current Openfish FISH ledger trading."
                }))?
            );
        }
    }

    Ok(())
}
