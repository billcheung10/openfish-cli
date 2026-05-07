use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use openfish_client_sdk::clob;
use openfish_client_sdk::clob::types::questions::{
    CreateQuestionRequest, ListClustersRequest, ListTemplatesRequest, RelatedMarketsRequest,
    ResolveQuestionRequest,
};
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::auth;
use crate::output::OutputFormat;
use crate::output::questions::{
    print_cluster, print_clusters, print_create_question, print_market_fees, print_related_markets,
    print_resolve_question, print_template, print_templates,
};

#[derive(Args)]
pub struct QuestionsArgs {
    #[command(subcommand)]
    pub command: QuestionsCommand,
}

#[derive(Subcommand)]
pub enum QuestionsCommand {
    /// List question templates
    Templates {
        /// Filter by category
        #[arg(long)]
        category: Option<String>,

        /// Search query
        #[arg(long)]
        search: Option<String>,

        /// Max results
        #[arg(long, default_value = "50")]
        limit: i64,

        /// Pagination offset
        #[arg(long, default_value = "0")]
        offset: i64,
    },

    /// Get a template by ID or slug
    Template {
        /// Template ID (UUID) or slug
        id: String,
    },

    /// List question clusters
    Clusters {
        /// Filter by template ID (UUID)
        #[arg(long)]
        template_id: Option<String>,

        /// Search query
        #[arg(long)]
        search: Option<String>,

        /// Max results
        #[arg(long, default_value = "50")]
        limit: i64,

        /// Pagination offset
        #[arg(long, default_value = "0")]
        offset: i64,
    },

    /// Get a cluster by ID
    Cluster {
        /// Cluster ID (UUID)
        id: String,
    },

    /// Create a new question/market
    Create {
        /// Cluster ID (UUID)
        #[arg(long)]
        cluster_id: String,

        /// Parameters as JSON string
        #[arg(long)]
        parameters: String,

        /// Outcomes (comma-separated, e.g. "Yes,No")
        #[arg(long)]
        outcomes: Option<String>,

        /// Bond amount in FISH
        #[arg(long)]
        bond_amount: Option<String>,
    },

    /// Submit a resolution for a market
    Resolve {
        /// Condition ID (0x-prefixed)
        #[arg(long)]
        condition_id: String,

        /// Winning token ID
        #[arg(long)]
        winning_token_id: String,

        /// Resolution source (default: "manual")
        #[arg(long)]
        resolution_source: Option<String>,

        /// Resolution evidence as JSON string
        #[arg(long)]
        resolution_evidence: Option<String>,
    },

    /// Get related markets
    Related {
        /// Condition ID (0x-prefixed)
        condition_id: String,

        /// Edge type filter (e.g. CONDITION_VARIANT)
        #[arg(long)]
        edge_type: Option<String>,

        /// Maximum related markets to return
        #[arg(long)]
        limit: Option<i64>,
    },

    /// Get fee breakdown for a market
    Fees {
        /// Condition ID (0x-prefixed)
        condition_id: String,
    },
}

pub async fn execute(
    args: QuestionsArgs,
    output: OutputFormat,
    private_key: Option<&str>,
    signature_type: Option<&str>,
) -> Result<()> {
    let output = &output;

    match args.command {
        QuestionsCommand::Templates {
            category,
            search,
            limit,
            offset,
        } => {
            let client = unauthenticated_client()?;
            let req = ListTemplatesRequest::builder()
                .maybe_category(category)
                .maybe_search(search)
                .limit(limit)
                .offset(offset)
                .build();
            let response = client.list_templates(&req).await?;
            print_templates(&response.templates, output)?;
        }

        QuestionsCommand::Template { id } => {
            let client = unauthenticated_client()?;
            let response = if let Ok(uuid) = id.parse::<Uuid>() {
                client.get_template(&uuid).await?
            } else {
                client.get_template_by_slug(&id).await?
            };
            print_template(&response.template, output)?;
        }

        QuestionsCommand::Clusters {
            template_id,
            search,
            limit,
            offset,
        } => {
            let client = unauthenticated_client()?;
            let tid = template_id
                .map(|s| s.parse::<Uuid>())
                .transpose()
                .context("Invalid UUID for --template-id")?;
            let req = ListClustersRequest::builder()
                .maybe_template_id(tid)
                .maybe_search(search)
                .limit(limit)
                .offset(offset)
                .build();
            let response = client.list_clusters(&req).await?;
            print_clusters(&response.clusters, output)?;
        }

        QuestionsCommand::Cluster { id } => {
            let client = unauthenticated_client()?;
            let uuid: Uuid = id.parse().context("Invalid UUID for cluster ID")?;
            let response = client.get_cluster(&uuid).await?;
            print_cluster(&response.cluster, output)?;
        }

        QuestionsCommand::Related {
            condition_id,
            edge_type,
            limit,
        } => {
            let client = unauthenticated_client()?;
            let req = RelatedMarketsRequest::builder()
                .maybe_edge_type(edge_type)
                .maybe_limit(limit)
                .build();
            let response = client.related_markets(&condition_id, &req).await?;
            print_related_markets(&response, output)?;
        }

        QuestionsCommand::Fees { condition_id } => {
            let client = unauthenticated_client()?;
            let response = client.market_fees(&condition_id).await?;
            print_market_fees(&response, output)?;
        }

        QuestionsCommand::Create {
            cluster_id,
            parameters,
            outcomes,
            bond_amount,
        } => {
            let client = auth::authenticated_clob_client(private_key, signature_type).await?;
            let cid: Uuid = cluster_id
                .parse()
                .context("Invalid UUID for --cluster-id")?;
            let params: serde_json::Value =
                serde_json::from_str(&parameters).context("Invalid JSON for --parameters")?;
            let outcomes = outcomes.map(|s| s.split(',').map(|o| o.trim().to_string()).collect());
            let bond = bond_amount
                .map(|s| s.parse::<Decimal>())
                .transpose()
                .context("Invalid decimal for --bond-amount")?;

            let req = CreateQuestionRequest::builder()
                .cluster_id(cid)
                .parameters(params)
                .maybe_outcomes(outcomes)
                .maybe_bond_amount(bond)
                .build();
            let response = client.create_question(&req).await?;
            print_create_question(&response, output)?;
        }

        QuestionsCommand::Resolve {
            condition_id,
            winning_token_id,
            resolution_source,
            resolution_evidence,
        } => {
            let client = auth::authenticated_clob_client(private_key, signature_type).await?;
            let evidence: Option<serde_json::Value> = resolution_evidence
                .map(|e| serde_json::from_str(&e))
                .transpose()
                .context("Invalid JSON for --resolution-evidence")?;

            let req = ResolveQuestionRequest::builder()
                .condition_id(condition_id)
                .winning_token_id(winning_token_id)
                .maybe_resolution_source(resolution_source)
                .maybe_resolution_evidence(evidence)
                .build();
            let response = client.resolve_question(&req).await?;
            print_resolve_question(&response, output)?;
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
