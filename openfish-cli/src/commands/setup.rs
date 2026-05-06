use std::io::{self, BufRead, Write};
use std::str::FromStr;

use anyhow::{Context, Result};
use openfish_client_sdk::DEFAULT_CHAIN_ID;
use openfish_client_sdk::auth::{LocalSigner, Signer as _};
use openfish_client_sdk::types::Address;

use crate::config;

fn print_banner() {
    // #2E5CFF → RGB(46, 92, 255)
    let b = "\x1b[38;2;46;92;255m";
    let bold = "\x1b[1m";
    let dim = "\x1b[2m";
    let r = "\x1b[0m";

    println!();

    println!("  {b}{bold} ██████╗ ██████╗ ███████╗███╗   ██╗███████╗██╗███████╗██╗  ██╗{r}");
    println!("  {b}{bold}██╔═══██╗██╔══██╗██╔════╝████╗  ██║██╔════╝██║██╔════╝██║  ██║{r}");
    println!("  {b}{bold}██║   ██║██████╔╝█████╗  ██╔██╗ ██║█████╗  ██║███████╗███████║{r}");
    println!("  {b}{bold}██║   ██║██╔═══╝ ██╔══╝  ██║╚██╗██║██╔══╝  ██║╚════██║██╔══██║{r}");
    println!("  {b}{bold}╚██████╔╝██║     ███████╗██║ ╚████║██║     ██║███████║██║  ██║{r}");
    println!("  {b}{bold} ╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═══╝╚═╝     ╚═╝╚══════╝╚═╝  ╚═╝{r}");

    println!();

    // Box width matches logo (83 chars)
    println!(
        "  {b}╭─────────────────────────────────────────────────────────────────────────────────╮{r}"
    );
    println!(
        "  {b}│{r}               {bold}Preview{r} {dim}— use small amounts only, at your own risk.{r}               {b}│{r}"
    );
    println!(
        "  {b}╰─────────────────────────────────────────────────────────────────────────────────╯{r}"
    );

    println!();
}

fn prompt(msg: &str) -> Result<String> {
    print!("{msg}");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_yn(msg: &str, default: bool) -> Result<bool> {
    let hint = if default { "Y/n" } else { "y/N" };
    let input = prompt(&format!("{msg} [{hint}] "))?;
    Ok(match input.to_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => default,
    })
}

fn step_header(n: u8, total: u8, label: &str) {
    println!("  [{n}/{total}] {label}");
    println!("  {}", "─".repeat(label.len() + 6));
}

pub fn execute() -> Result<()> {
    print_banner();

    let total = 4;

    step_header(1, total, "Wallet");

    let address = if config::config_exists() {
        let (key, source) = config::resolve_key(None)?;
        if let Some(k) = &key
            && let Ok(signer) = LocalSigner::from_str(k)
        {
            let addr = signer.address();
            println!("  ✓ Wallet already configured ({})", source.label());
            println!("    Address: {addr}");
            println!();

            if !prompt_yn("  Reconfigure wallet?", false)? {
                finish_setup(addr)?;
                return Ok(());
            }
            println!();
        }
        setup_wallet()?
    } else {
        setup_wallet()?
    };

    println!();

    finish_setup(address)
}

fn setup_wallet() -> Result<Address> {
    let has_key = prompt_yn("  Do you have an existing private key?", false)?;

    let (address, key_hex) = if has_key {
        let key = prompt("  Enter private key: ")?;
        let signer = LocalSigner::from_str(&key)
            .context("Invalid private key")?
            .with_chain_id(Some(DEFAULT_CHAIN_ID));
        let hex = format!("{:#x}", signer.to_bytes());
        (signer.address(), hex)
    } else {
        let signer = LocalSigner::random().with_chain_id(Some(DEFAULT_CHAIN_ID));
        let address = signer.address();
        let hex = format!("{:#x}", signer.to_bytes());
        (address, hex)
    };

    config::save_wallet(&key_hex, DEFAULT_CHAIN_ID, config::DEFAULT_SIGNATURE_TYPE)?;

    if has_key {
        println!("  ✓ Wallet imported");
    } else {
        println!("  ✓ Wallet created");
    }
    println!("    Address: {address}");
    println!("    Config:  {}", config::config_path()?.display());

    if !has_key {
        println!();
        println!("  ⚠ Back up your private key from the config file.");
        println!("    If lost, your funds cannot be recovered.");
    }

    Ok(address)
}

fn finish_setup(address: Address) -> Result<()> {
    let total = 4;

    step_header(2, total, "Openfish Wallet");

    println!("  ✓ Wallet ready for Openfish API authentication");
    println!("    Address: {address}");
    println!("    Chain:   BSC (56)");
    println!("    Mode:    EOA signatures");

    println!();

    step_header(3, total, "Fund Wallet");

    println!("  ○ Bridge FISH from BSC into your Openfish ledger");
    println!("    Run: openfish bridge deposit {address}");
    println!("    Send only FISH on BSC to the returned deposit address.");

    println!();

    step_header(4, total, "Trading Readiness");

    println!("  No on-chain approvals are required for current FISH ledger trading.");
    println!("  After funding, run `openfish clob balance --asset-type collateral`.");

    println!();
    println!("  ────────────────────────────────────");
    println!("  ✓ Setup complete! You're ready to go.");
    println!();
    println!("  Next steps:");
    println!("    openfish shell              Interactive mode");
    println!("    openfish markets list        Browse markets");
    println!("    openfish clob book <token>   View order book");
    println!();

    Ok(())
}
