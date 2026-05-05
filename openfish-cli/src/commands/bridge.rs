use anyhow::Result;
use clap::{Args, Subcommand};
use openfish_client_sdk::bridge::{
    self,
    types::{
        DepositRequest, QuoteRequest, StatusRequest, SwapExecuteRequest, SwapQuoteRequest,
        WithdrawRequest,
    },
};

use crate::output::OutputFormat;
use crate::output::bridge::{
    print_deposit, print_quote, print_status, print_supported_assets, print_swap_execute,
    print_swap_list, print_swap_quote, print_swap_status, print_withdraw, print_withdraw_preview,
};

#[derive(Args)]
pub struct BridgeArgs {
    #[command(subcommand)]
    pub command: BridgeCommand,
}

#[derive(Subcommand)]
pub enum BridgeCommand {
    /// Get the FISH deposit address for a wallet
    Deposit {
        /// Openfish wallet address (0x...)
        address: openfish_client_sdk::types::Address,
    },

    /// Withdraw FISH from Openfish to BSC
    Withdraw {
        /// Source Openfish wallet address (0x...)
        address: openfish_client_sdk::types::Address,
        /// Destination chain ID. Use 56 for BSC.
        #[arg(long)]
        to_chain_id: u64,
        /// Destination token contract address
        #[arg(long)]
        to_token_address: String,
        /// Destination wallet address where funds will be sent
        #[arg(long)]
        recipient: String,
        /// Amount in base units. Omit or "all" for full balance.
        #[arg(long, default_value = "all")]
        amount: String,
        /// Preview amount, fee, total debit, and hot-wallet liquidity without submitting
        #[arg(long)]
        preview: bool,
    },

    /// List supported chains and tokens for FISH deposits and withdrawals
    SupportedAssets,

    /// Check deposit and withdrawal status for an address
    Status {
        /// Deposit address (EVM, Solana, or Bitcoin)
        address: String,
    },

    /// Get an estimated quote for a deposit or withdrawal
    Quote {
        /// Amount in base units (without decimals)
        #[arg(long)]
        amount: String,
        /// Source chain ID
        #[arg(long)]
        from_chain_id: u64,
        /// Source token address
        #[arg(long)]
        from_token_address: String,
        /// Recipient address
        #[arg(long)]
        recipient: String,
        /// Destination chain ID
        #[arg(long)]
        to_chain_id: u64,
        /// Destination token address
        #[arg(long)]
        to_token_address: String,
    },

    /// User-controlled BNB -> FISH swap
    Swap {
        #[command(subcommand)]
        command: SwapCommand,
    },
}

#[derive(Subcommand)]
pub enum SwapCommand {
    /// Quote a BNB -> FISH swap without sending a transaction
    Quote {
        /// Openfish wallet address (0x...)
        address: openfish_client_sdk::types::Address,
        /// BNB amount in wei
        #[arg(long)]
        amount_in_wei: String,
        /// FISH token contract address
        #[arg(long)]
        to_token_address: String,
        /// Optional slippage in basis points
        #[arg(long)]
        slippage_bps: Option<u16>,
    },

    /// Execute a quoted BNB -> FISH swap
    Execute {
        /// Openfish wallet address (0x...)
        address: openfish_client_sdk::types::Address,
        /// Quote id from `openfish bridge swap quote`
        #[arg(long)]
        quote_id: String,
    },

    /// Get swap status
    Status {
        /// Swap id from `openfish bridge swap execute`
        swap_id: String,
    },

    /// List swap history for an Openfish wallet
    List {
        /// Openfish wallet address (0x...)
        address: openfish_client_sdk::types::Address,
        /// Maximum rows to return
        #[arg(long)]
        limit: Option<i64>,
    },
}

pub async fn execute(
    client: &bridge::Client,
    args: BridgeArgs,
    output: OutputFormat,
) -> Result<()> {
    match args.command {
        BridgeCommand::Deposit { address } => {
            let request = DepositRequest::builder().address(address).build();

            let response = client.deposit(&request).await?;
            print_deposit(&response, &output)?;
        }

        BridgeCommand::Withdraw {
            address,
            to_chain_id,
            to_token_address,
            recipient,
            amount,
            preview,
        } => {
            let amount_opt = if amount.eq_ignore_ascii_case("all") {
                None
            } else {
                Some(amount)
            };

            let request = WithdrawRequest::builder()
                .address(address)
                .to_chain_id(to_chain_id)
                .to_token_address(&to_token_address)
                .recipient_addr(&recipient)
                .maybe_amount(amount_opt)
                .build();

            if preview {
                let response = client.withdraw_preview(&request).await?;
                print_withdraw_preview(&response, &output)?;
            } else {
                let response = client.withdraw(&request).await?;
                print_withdraw(&response, &output)?;
            }
        }

        BridgeCommand::SupportedAssets => {
            let response = client.supported_assets().await?;
            print_supported_assets(&response, &output)?;
        }

        BridgeCommand::Status { address } => {
            anyhow::ensure!(!address.trim().is_empty(), "Address cannot be empty");
            let request = StatusRequest::builder().address(&address).build();

            let response = client.status(&request).await?;
            print_status(&response, &output)?;
        }

        BridgeCommand::Quote {
            amount,
            from_chain_id,
            from_token_address,
            recipient,
            to_chain_id,
            to_token_address,
        } => {
            let from_amount: alloy::primitives::U256 = amount
                .parse()
                .map_err(|_| anyhow::anyhow!("Invalid amount: {amount}"))?;

            let request = QuoteRequest::builder()
                .from_amount_base_unit(from_amount)
                .from_chain_id(from_chain_id)
                .from_token_address(&from_token_address)
                .recipient_address(&recipient)
                .to_chain_id(to_chain_id)
                .to_token_address(&to_token_address)
                .build();

            let response = client.quote(&request).await?;
            print_quote(&response, &output)?;
        }

        BridgeCommand::Swap { command } => match command {
            SwapCommand::Quote {
                address,
                amount_in_wei,
                to_token_address,
                slippage_bps,
            } => {
                let request = SwapQuoteRequest::builder()
                    .address(address)
                    .from_token("BNB")
                    .to_token_address(to_token_address)
                    .amount_in_wei(amount_in_wei)
                    .maybe_slippage_bps(slippage_bps)
                    .build();

                let response = client.swap_quote(&request).await?;
                print_swap_quote(&response, &output)?;
            }
            SwapCommand::Execute { address, quote_id } => {
                let request = SwapExecuteRequest::builder()
                    .address(address)
                    .quote_id(quote_id)
                    .build();

                let response = client.swap_execute(&request).await?;
                print_swap_execute(&response, &output)?;
            }
            SwapCommand::Status { swap_id } => {
                let response = client.swap_status(&swap_id).await?;
                print_swap_status(&response, &output)?;
            }
            SwapCommand::List { address, limit } => {
                let response = client.swap_list(&address.to_string(), limit).await?;
                print_swap_list(&response, &output)?;
            }
        },
    }

    Ok(())
}
