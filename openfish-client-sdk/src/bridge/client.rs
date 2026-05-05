use reqwest::{
    Client as ReqwestClient, Method,
    header::{HeaderMap, HeaderValue},
};
use url::Url;

use super::types::{
    DepositRequest, DepositResponse, QuoteRequest, QuoteResponse, StatusRequest, StatusResponse,
    SupportedAssetsResponse, SwapExecuteRequest, SwapExecuteResponse, SwapQuoteRequest,
    SwapQuoteResponse, SwapStatusResponse, WithdrawPreviewResponse, WithdrawRequest,
    WithdrawResponse,
};
use crate::Result;

/// Client for the Openfish Bridge API.
///
/// The current Openfish Bridge API supports FISH deposits and withdrawals.
///
/// # Example
///
/// ```no_run
/// use openfish_client_sdk::types::address;
/// use openfish_client_sdk::bridge::{Client, types::DepositRequest};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::default();
///
/// // Get deposit addresses
/// let request = DepositRequest::builder()
///     .address(address!("56687bf447db6ffa42ffe2204a05edaa20f55839"))
///     .build();
/// let response = client.deposit(&request).await?;
///
/// // Get supported assets
/// let assets = client.supported_assets().await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct Client {
    host: Url,
    client: ReqwestClient,
}

impl Default for Client {
    fn default() -> Self {
        Client::new("https://bridge.openfish.com")
            .expect("Client with default endpoint should succeed")
    }
}

impl Client {
    /// Creates a new Bridge API client with a custom host.
    ///
    /// # Errors
    ///
    /// Returns an error if the host URL is invalid or the HTTP client fails to build.
    pub fn new(host: &str) -> Result<Client> {
        let mut headers = HeaderMap::new();

        headers.insert("User-Agent", HeaderValue::from_static("rs_clob_client"));
        headers.insert("Accept", HeaderValue::from_static("*/*"));
        headers.insert("Connection", HeaderValue::from_static("keep-alive"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        let client = ReqwestClient::builder().default_headers(headers).build()?;

        Ok(Self {
            host: Url::parse(host)?,
            client,
        })
    }

    /// Returns the host URL for the client.
    #[must_use]
    pub fn host(&self) -> &Url {
        &self.host
    }

    #[must_use]
    fn client(&self) -> &ReqwestClient {
        &self.client
    }

    /// Create deposit addresses for a Openfish wallet.
    ///
    /// Generates unique deposit addresses for bridging assets to Openfish.
    /// Returns addresses for EVM-compatible chains, Solana, and Bitcoin.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openfish_client_sdk::types::address;
    /// use openfish_client_sdk::bridge::{Client, types::DepositRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::default();
    /// let request = DepositRequest::builder()
    ///     .address(address!("56687bf447db6ffa42ffe2204a05edaa20f55839"))
    ///     .build();
    ///
    /// let response = client.deposit(&request).await?;
    /// println!("EVM: {}", response.address.evm);
    /// println!("SVM: {:?}", response.address.svm);
    /// println!("BTC: {:?}", response.address.btc);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn deposit(&self, request: &DepositRequest) -> Result<DepositResponse> {
        let request = self
            .client()
            .request(Method::POST, format!("{}deposit", self.host()))
            .json(request)
            .build()?;

        crate::request(&self.client, request, None).await
    }

    /// Withdraw FISH from an Openfish wallet to a supported external address.
    pub async fn withdraw(&self, request: &WithdrawRequest) -> Result<WithdrawResponse> {
        let request = self
            .client()
            .request(Method::POST, format!("{}withdraw", self.host()))
            .json(request)
            .build()?;

        crate::request(&self.client, request, None).await
    }

    /// Preview a FISH withdrawal before submitting it.
    pub async fn withdraw_preview(
        &self,
        request: &WithdrawRequest,
    ) -> Result<WithdrawPreviewResponse> {
        let request = self
            .client()
            .request(Method::POST, format!("{}withdraw/preview", self.host()))
            .json(request)
            .build()?;

        crate::request(&self.client, request, None).await
    }

    /// Get all supported chains and tokens for deposits.
    ///
    /// Returns information about which assets can be deposited and their
    /// minimum deposit amounts in USD.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openfish_client_sdk::bridge::Client;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::default();
    /// let response = client.supported_assets().await?;
    ///
    /// for asset in response.supported_assets {
    ///     println!(
    ///         "{} ({}) on {} - min: ${:.2}",
    ///         asset.token.name,
    ///         asset.token.symbol,
    ///         asset.chain_name,
    ///         asset.min_checkout_usd
    ///     );
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn supported_assets(&self) -> Result<SupportedAssetsResponse> {
        let request = self
            .client()
            .request(Method::GET, format!("{}supported-assets", self.host()))
            .build()?;

        crate::request(&self.client, request, None).await
    }

    /// Get the transaction status for all deposits associated with a given deposit address.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use openfish_client_sdk::bridge::{Client, types::StatusRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::default();
    ///
    /// let request = StatusRequest::builder()
    ///     .address("56687bf447db6ffa42ffe2204a05edaa20f55839")
    ///     .build();
    /// let response = client.status(&request).await?;
    ///
    /// for tx in response.transactions {
    ///     println!(
    ///         "Sent {} amount of token {} on chainId {} to destination chainId {} with status {:?}",
    ///         tx.from_amount_base_unit,
    ///         tx.from_token_address,
    ///         tx.from_chain_id,
    ///         tx.to_chain_id,
    ///         tx.status
    ///     );
    /// }
    /// # Ok(())
    /// # }
    ///
    /// ```
    pub async fn status(&self, request: &StatusRequest) -> Result<StatusResponse> {
        let request = self
            .client()
            .request(
                Method::GET,
                format!("{}status/{}", self.host(), request.address),
            )
            .build()?;

        crate::request(&self.client, request, None).await
    }

    /// Get an estimated quote for a deposit or withdrawal,
    /// including output amounts, checkout time, and a detailed fee breakdown.
    pub async fn quote(&self, request: &QuoteRequest) -> Result<QuoteResponse> {
        let request = self
            .client()
            .request(Method::POST, format!("{}quote", self.host()))
            .json(request)
            .build()?;

        crate::request(&self.client, request, None).await
    }

    /// Quote a user-confirmed BNB -> FISH swap.
    pub async fn swap_quote(&self, request: &SwapQuoteRequest) -> Result<SwapQuoteResponse> {
        let request = self
            .client()
            .request(Method::POST, format!("{}bridge/swap/quote", self.host()))
            .json(request)
            .build()?;

        crate::request(&self.client, request, None).await
    }

    /// Execute a previously quoted BNB -> FISH swap.
    pub async fn swap_execute(&self, request: &SwapExecuteRequest) -> Result<SwapExecuteResponse> {
        let request = self
            .client()
            .request(Method::POST, format!("{}bridge/swap/execute", self.host()))
            .json(request)
            .build()?;

        crate::request(&self.client, request, None).await
    }

    /// Get BNB -> FISH swap status.
    pub async fn swap_status(&self, swap_id: &str) -> Result<SwapStatusResponse> {
        let request = self
            .client()
            .request(Method::GET, format!("{}bridge/swap/{swap_id}", self.host()))
            .build()?;

        crate::request(&self.client, request, None).await
    }

    /// List BNB -> FISH swaps for an Openfish wallet.
    pub async fn swap_list(
        &self,
        address: &str,
        limit: Option<i64>,
    ) -> Result<Vec<SwapStatusResponse>> {
        let mut url = self.host().join("bridge/swaps")?;
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("address", address);
            if let Some(limit) = limit {
                query.append_pair("limit", &limit.to_string());
            }
        }
        let request = self.client().request(Method::GET, url).build()?;

        crate::request(&self.client, request, None).await
    }
}
