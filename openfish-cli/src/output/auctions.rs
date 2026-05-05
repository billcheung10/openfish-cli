use openfish_client_sdk::clob::types::questions::{
    AuctionBidResponse, GetAuctionResponse, ProposeQuestionResponse, QuestionAuction,
};
use tabled::settings::Style;
use tabled::{Table, Tabled};

use super::{
    DASH, OutputFormat, detail_field, format_date, format_decimal, print_detail_table, print_json,
    truncate,
};

// ─── Auction List ───

#[derive(Tabled)]
struct AuctionRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Cluster")]
    cluster: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Proposer")]
    proposer: String,
    #[tabled(rename = "End At")]
    end_at: String,
    #[tabled(rename = "Winner Fee Rate")]
    winner_fee_rate: String,
}

fn auction_to_row(a: &QuestionAuction) -> AuctionRow {
    AuctionRow {
        id: a.id.to_string(),
        cluster: truncate(&a.cluster_id.to_string(), 20),
        status: a.status.clone(),
        proposer: a
            .proposer_agent
            .as_deref()
            .map(|s| truncate(s, 20))
            .unwrap_or_else(|| DASH.to_string()),
        end_at: a
            .end_at
            .as_ref()
            .map(|d| format_date(d))
            .unwrap_or_else(|| DASH.to_string()),
        winner_fee_rate: a
            .winner_fee_rate
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string()),
    }
}

pub fn print_auctions(auctions: &[QuestionAuction], output: &OutputFormat) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            if auctions.is_empty() {
                println!("No auctions found.");
                return Ok(());
            }
            let rows: Vec<AuctionRow> = auctions.iter().map(auction_to_row).collect();
            let table = Table::new(rows).with(Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => print_json(auctions)?,
    }
    Ok(())
}

// ─── Auction Detail ───

#[derive(Tabled)]
struct BidRow {
    #[tabled(rename = "Bid ID")]
    bid_id: String,
    #[tabled(rename = "Agent")]
    agent: String,
    #[tabled(rename = "Fee Rate")]
    fee_rate: String,
    #[tabled(rename = "Bond")]
    bond: String,
    #[tabled(rename = "Created")]
    created: String,
}

pub fn print_auction(response: &GetAuctionResponse, output: &OutputFormat) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(response);
    }

    let a = &response.auction;
    let mut rows: Vec<[String; 2]> = Vec::new();

    detail_field!(rows, "ID", a.id.to_string());
    detail_field!(rows, "Cluster ID", a.cluster_id.to_string());
    detail_field!(
        rows,
        "Parameters",
        serde_json::to_string_pretty(&a.parameters).unwrap_or_default()
    );
    detail_field!(
        rows,
        "Parameters Hash",
        a.parameters_hash.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(rows, "Status", a.status.clone());
    detail_field!(
        rows,
        "Proposer",
        a.proposer_agent.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(
        rows,
        "Start At",
        a.start_at
            .as_ref()
            .map(|d| format_date(d))
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "End At",
        a.end_at
            .as_ref()
            .map(|d| format_date(d))
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Winner Agent",
        a.winner_agent.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(
        rows,
        "Winner Fee Rate",
        a.winner_fee_rate
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Winner Bond",
        a.winner_bond
            .map(|d| format_decimal(d))
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Condition ID",
        a.condition_id.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(rows, "Created At", format_date(&a.created_at));
    detail_field!(rows, "Updated At", format_date(&a.updated_at));

    print_detail_table(rows);

    // Print bids table
    if !response.bids.is_empty() {
        println!("\nBids ({}):", response.bid_count);
        let bid_rows: Vec<BidRow> = response
            .bids
            .iter()
            .map(|b| BidRow {
                bid_id: b.id.to_string(),
                agent: truncate(&b.agent_address, 20),
                fee_rate: b.proposed_fee_rate.to_string(),
                bond: format_decimal(b.bond_amount),
                created: format_date(&b.created_at),
            })
            .collect();
        let table = Table::new(bid_rows).with(Style::rounded()).to_string();
        println!("{table}");
    }

    Ok(())
}

// ─── Propose Result ───

pub fn print_propose_result(
    response: &ProposeQuestionResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(response);
    }
    let mut rows: Vec<[String; 2]> = Vec::new();

    detail_field!(rows, "Auction ID", response.auction_id.to_string());
    detail_field!(rows, "Action", response.action.clone());
    detail_field!(
        rows,
        "Status",
        response.status.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(
        rows,
        "End At",
        response
            .end_at
            .as_ref()
            .map(|d| format_date(d))
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Fee Rate",
        response
            .proposed_fee_rate
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Bond",
        response
            .bond_amount
            .map(|d| format_decimal(d))
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Cluster ID",
        response
            .cluster_id
            .map(|id| id.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Cluster Slug",
        response.cluster_slug.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(
        rows,
        "Template Slug",
        response
            .template_slug
            .as_deref()
            .unwrap_or(DASH)
            .to_string()
    );
    detail_field!(
        rows,
        "Message",
        response.message.as_deref().unwrap_or(DASH).to_string()
    );

    print_detail_table(rows);
    Ok(())
}

// ─── Bid Result ───

pub fn print_bid_result(
    response: &AuctionBidResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(response);
    }
    let mut rows: Vec<[String; 2]> = Vec::new();

    detail_field!(rows, "Auction ID", response.auction_id.to_string());
    detail_field!(rows, "Bid ID", response.bid_id.to_string());
    detail_field!(rows, "Fee Rate", response.proposed_fee_rate.to_string());
    detail_field!(rows, "Bond", format_decimal(response.bond_amount));
    detail_field!(
        rows,
        "Auction End At",
        response
            .auction_end_at
            .as_ref()
            .map(|d| format_date(d))
            .unwrap_or_else(|| DASH.to_string())
    );

    print_detail_table(rows);
    Ok(())
}
