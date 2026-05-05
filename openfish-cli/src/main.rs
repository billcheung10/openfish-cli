mod auth;
mod commands;
mod config;
mod output;
mod shell;

use std::process::ExitCode;

use clap::{Parser, Subcommand};
use output::OutputFormat;

#[derive(Parser)]
#[command(name = "openfish", about = "Openfish CLI", version)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output format: table or json
    #[arg(short, long, global = true, default_value = "table")]
    pub(crate) output: OutputFormat,

    /// Private key (overrides env var and config file)
    #[arg(long, global = true)]
    private_key: Option<String>,

    /// Signature type: eoa, proxy, or gnosis-safe
    #[arg(long, global = true)]
    signature_type: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Guided first-time setup (wallet, proxy, approvals)
    Setup,
    /// Launch interactive shell
    Shell,
    /// Interact with markets
    Markets(commands::markets::MarketsArgs),
    /// Interact with events
    Events(commands::events::EventsArgs),
    /// Interact with tags
    Tags(commands::tags::TagsArgs),
    /// Interact with series
    Series(commands::series::SeriesArgs),
    /// Interact with comments
    Comments(commands::comments::CommentsArgs),
    /// Look up public profiles
    Profiles(commands::profiles::ProfilesArgs),
    /// Sports metadata and teams
    Sports(commands::sports::SportsArgs),
    /// Check and set contract approvals for trading
    Approve(commands::approve::ApproveArgs),
    /// Interact with the CLOB (order book, trading, balances)
    Clob(commands::clob::ClobArgs),
    /// CTF operations: split, merge, redeem positions
    Ctf(commands::ctf::CtfArgs),
    /// Query on-chain data (positions, trades, leaderboards)
    Data(commands::data::DataArgs),
    /// Bridge assets from other chains to Openfish
    Bridge(commands::bridge::BridgeArgs),
    /// Manage wallet and authentication
    Wallet(commands::wallet::WalletArgs),
    /// Question templates, clusters, creation, and resolution
    Questions(commands::questions::QuestionsArgs),
    /// Fee rate auctions for question creation
    Auctions(commands::auctions::AuctionsArgs),
    /// Agent bonds for market resolution quality
    Bonds(commands::bonds::BondsArgs),
    /// Dispute pending market resolutions
    Disputes(commands::disputes::DisputesArgs),
    /// Check API health status
    Status,
    /// Update to the latest version
    Upgrade,
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let output = cli.output;

    if let Err(e) = run(cli).await {
        output::print_error(&e, output);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[allow(clippy::too_many_lines)]
pub(crate) async fn run(cli: Cli) -> anyhow::Result<()> {
    // Lazy-init so we only pay for the client we actually use.
    // Support custom hosts via env vars for local/mock backend testing.
    let gamma = std::cell::LazyCell::new(|| match std::env::var("OPENFISH_GAMMA_HOST") {
        Ok(h) if !h.is_empty() => {
            openfish_client_sdk::gamma::Client::new(&h).expect("Invalid OPENFISH_GAMMA_HOST")
        }
        _ => openfish_client_sdk::gamma::Client::default(),
    });
    let data = std::cell::LazyCell::new(|| match std::env::var("OPENFISH_DATA_HOST") {
        Ok(h) if !h.is_empty() => {
            openfish_client_sdk::data::Client::new(&h).expect("Invalid OPENFISH_DATA_HOST")
        }
        _ => openfish_client_sdk::data::Client::default(),
    });
    let bridge = std::cell::LazyCell::new(|| match std::env::var("OPENFISH_BRIDGE_HOST") {
        Ok(h) if !h.is_empty() => {
            openfish_client_sdk::bridge::Client::new(&h).expect("Invalid OPENFISH_BRIDGE_HOST")
        }
        _ => openfish_client_sdk::bridge::Client::default(),
    });

    match cli.command {
        Commands::Setup => commands::setup::execute(),
        Commands::Shell => Box::pin(shell::run_shell()).await,
        Commands::Markets(args) => commands::markets::execute(&gamma, args, cli.output).await,
        Commands::Events(args) => commands::events::execute(&gamma, args, cli.output).await,
        Commands::Tags(args) => commands::tags::execute(&gamma, args, cli.output).await,
        Commands::Series(args) => commands::series::execute(&gamma, args, cli.output).await,
        Commands::Comments(args) => commands::comments::execute(&gamma, args, cli.output).await,
        Commands::Profiles(args) => commands::profiles::execute(&gamma, args, cli.output).await,
        Commands::Sports(args) => commands::sports::execute(&gamma, args, cli.output).await,
        Commands::Approve(args) => {
            commands::approve::execute(args, cli.output, cli.private_key.as_deref()).await
        }
        Commands::Clob(args) => {
            commands::clob::execute(
                args,
                cli.output,
                cli.private_key.as_deref(),
                cli.signature_type.as_deref(),
            )
            .await
        }
        Commands::Ctf(args) => {
            commands::ctf::execute(args, cli.output, cli.private_key.as_deref()).await
        }
        Commands::Data(args) => commands::data::execute(&data, args, cli.output).await,
        Commands::Bridge(args) => commands::bridge::execute(&bridge, args, cli.output).await,
        Commands::Wallet(args) => {
            commands::wallet::execute(args, cli.output, cli.private_key.as_deref())
        }
        Commands::Questions(args) => {
            commands::questions::execute(
                args,
                cli.output,
                cli.private_key.as_deref(),
                cli.signature_type.as_deref(),
            )
            .await
        }
        Commands::Auctions(args) => {
            commands::auctions::execute(
                args,
                cli.output,
                cli.private_key.as_deref(),
                cli.signature_type.as_deref(),
            )
            .await
        }
        Commands::Bonds(args) => {
            commands::bonds::execute(
                args,
                cli.output,
                cli.private_key.as_deref(),
                cli.signature_type.as_deref(),
            )
            .await
        }
        Commands::Disputes(args) => {
            commands::disputes::execute(
                args,
                cli.output,
                cli.private_key.as_deref(),
                cli.signature_type.as_deref(),
            )
            .await
        }
        Commands::Upgrade => commands::upgrade::execute(),
        Commands::Status => {
            let status = gamma.status().await?;
            match cli.output {
                OutputFormat::Json => {
                    println!("{}", serde_json::json!({"status": status}));
                }
                OutputFormat::Table => {
                    println!("API Status: {status}");
                }
            }
            Ok(())
        }
    }
}
