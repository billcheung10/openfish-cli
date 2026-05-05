use std::str::FromStr;

use alloy::providers::ProviderBuilder;
use anyhow::{Context, Result};
use openfish_client_sdk::auth::state::Authenticated;
use openfish_client_sdk::auth::{LocalSigner, Normal, Signer as _};
use openfish_client_sdk::clob::types::SignatureType;
use openfish_client_sdk::{POLYGON, clob};

use crate::config;

const DEFAULT_RPC_URL: &str = "https://polygon.drpc.org";

fn rpc_url() -> String {
    std::env::var("OPENFISH_RPC_URL").unwrap_or_else(|_| DEFAULT_RPC_URL.to_string())
}

fn parse_signature_type(s: &str) -> SignatureType {
    match s {
        config::DEFAULT_SIGNATURE_TYPE => SignatureType::Proxy,
        "gnosis-safe" => SignatureType::GnosisSafe,
        _ => SignatureType::Eoa,
    }
}

pub fn resolve_signer(private_key: Option<&str>) -> Result<impl openfish_client_sdk::auth::Signer> {
    let (key, _) = config::resolve_key(private_key)?;
    let key = key.ok_or_else(|| anyhow::anyhow!("{}", config::NO_WALLET_MSG))?;
    LocalSigner::from_str(&key)
        .context("Invalid private key")
        .map(|s| s.with_chain_id(Some(POLYGON)))
}

pub async fn authenticated_clob_client(
    private_key: Option<&str>,
    signature_type_flag: Option<&str>,
) -> Result<clob::Client<Authenticated<Normal>>> {
    let signer = resolve_signer(private_key)?;
    authenticate_with_signer(&signer, signature_type_flag).await
}

pub async fn authenticate_with_signer(
    signer: &(impl openfish_client_sdk::auth::Signer + Sync),
    signature_type_flag: Option<&str>,
) -> Result<clob::Client<Authenticated<Normal>>> {
    let sig_type = parse_signature_type(&config::resolve_signature_type(signature_type_flag)?);

    let client = match std::env::var("OPENFISH_CLOB_HOST") {
        Ok(h) if !h.is_empty() => clob::Client::new(&h, clob::Config::default())?,
        _ => clob::Client::default(),
    };
    client
        .authentication_builder(signer)
        .signature_type(sig_type)
        .authenticate()
        .await
        .context("Failed to authenticate with Openfish CLOB")
}

pub async fn create_readonly_provider() -> Result<impl alloy::providers::Provider + Clone> {
    ProviderBuilder::new()
        .connect(&rpc_url())
        .await
        .context("Failed to connect to Polygon RPC")
}

pub async fn create_provider(
    private_key: Option<&str>,
) -> Result<impl alloy::providers::Provider + Clone> {
    let (key, _) = config::resolve_key(private_key)?;
    let key = key.ok_or_else(|| anyhow::anyhow!("{}", config::NO_WALLET_MSG))?;
    let signer = LocalSigner::from_str(&key)
        .context("Invalid private key")?
        .with_chain_id(Some(POLYGON));
    ProviderBuilder::new()
        .wallet(signer)
        .connect(&rpc_url())
        .await
        .context("Failed to connect to Polygon RPC with wallet")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_signature_type_proxy() {
        assert_eq!(parse_signature_type("proxy"), SignatureType::Proxy);
    }

    #[test]
    fn parse_signature_type_gnosis_safe() {
        assert_eq!(
            parse_signature_type("gnosis-safe"),
            SignatureType::GnosisSafe
        );
    }

    #[test]
    fn parse_signature_type_eoa() {
        assert_eq!(parse_signature_type("eoa"), SignatureType::Eoa);
    }

    #[test]
    fn parse_signature_type_unknown_defaults_to_eoa() {
        assert_eq!(parse_signature_type("unknown"), SignatureType::Eoa);
    }
}
