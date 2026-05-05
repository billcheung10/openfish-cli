use anyhow::Result;
use clap::{Args, Subcommand};
use openfish_client_sdk::clob::types::questions::DisputeRequest;

use crate::auth;
use crate::output::OutputFormat;
use crate::output::disputes::print_dispute_result;

#[derive(Args)]
pub struct DisputesArgs {
    #[command(subcommand)]
    pub command: DisputesCommand,
}

#[derive(Subcommand)]
pub enum DisputesCommand {
    /// Dispute a pending market resolution
    Create {
        /// Condition ID (0x-prefixed) of the market to dispute
        #[arg(long)]
        condition_id: String,
    },
}

pub async fn execute(
    args: DisputesArgs,
    output: OutputFormat,
    private_key: Option<&str>,
    signature_type: Option<&str>,
) -> Result<()> {
    let output = &output;

    match args.command {
        DisputesCommand::Create { condition_id } => {
            let client = auth::authenticated_clob_client(private_key, signature_type).await?;
            let req = DisputeRequest::builder().condition_id(condition_id).build();
            let response = client.dispute_resolution(&req).await?;
            print_dispute_result(&response, output)?;
        }
    }

    Ok(())
}
