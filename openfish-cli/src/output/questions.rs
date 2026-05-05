use openfish_client_sdk::clob::types::questions::{
    CreateQuestionResponse, MarketFeesResponse, QuestionCluster, QuestionTemplate,
    RelatedMarketsResponse, ResolveQuestionResponse,
};
use tabled::settings::Style;
use tabled::{Table, Tabled};

use super::{
    DASH, OutputFormat, detail_field, format_date, format_decimal, print_detail_table, print_json,
    truncate,
};

// ─── Templates ───

#[derive(Tabled)]
struct TemplateRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Slug")]
    slug: String,
    #[tabled(rename = "Category")]
    category: String,
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "Outcome Type")]
    outcome_type: String,
    #[tabled(rename = "Status")]
    status: String,
}

fn template_to_row(t: &QuestionTemplate) -> TemplateRow {
    TemplateRow {
        id: t.id.to_string(),
        slug: truncate(&t.slug, 30),
        category: t.category.as_deref().unwrap_or(DASH).to_string(),
        title: truncate(&t.title, 50),
        outcome_type: t.outcome_type.clone(),
        status: t.status.clone(),
    }
}

pub fn print_templates(
    templates: &[QuestionTemplate],
    output: &OutputFormat,
) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            if templates.is_empty() {
                println!("No templates found.");
                return Ok(());
            }
            let rows: Vec<TemplateRow> = templates.iter().map(template_to_row).collect();
            let table = Table::new(rows).with(Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => print_json(templates)?,
    }
    Ok(())
}

pub fn print_template(template: &QuestionTemplate, output: &OutputFormat) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(template);
    }
    let mut rows: Vec<[String; 2]> = Vec::new();

    detail_field!(rows, "ID", template.id.to_string());
    detail_field!(rows, "Slug", template.slug.clone());
    detail_field!(rows, "Version", template.version.to_string());
    detail_field!(
        rows,
        "Category",
        template.category.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(
        rows,
        "Subcategory",
        template.subcategory.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(rows, "Title", template.title.clone());
    detail_field!(
        rows,
        "Description",
        template.description.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(rows, "Status", template.status.clone());
    detail_field!(rows, "Question Format", template.question_format.clone());
    detail_field!(
        rows,
        "Parameter Schema",
        serde_json::to_string_pretty(&template.parameter_schema).unwrap_or_default()
    );
    detail_field!(rows, "Outcome Type", template.outcome_type.clone());
    detail_field!(
        rows,
        "Outcome Schema",
        template
            .outcome_schema
            .as_ref()
            .map(|v| serde_json::to_string_pretty(v).unwrap_or_default())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Resolution Type",
        template
            .resolution_type
            .as_deref()
            .unwrap_or(DASH)
            .to_string()
    );
    detail_field!(
        rows,
        "Resolution Config",
        template
            .resolution_config
            .as_ref()
            .map(|v| serde_json::to_string_pretty(v).unwrap_or_default())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Default Tick Size",
        template
            .default_tick_size
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Default Fee BPS",
        template
            .default_fee_bps
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Default Duration (days)",
        template
            .default_duration_days
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Tags",
        template
            .tags
            .as_ref()
            .map(|v| serde_json::to_string_pretty(v).unwrap_or_default())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Data",
        template
            .data
            .as_ref()
            .map(|v| serde_json::to_string_pretty(v).unwrap_or_default())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(rows, "Created At", format_date(&template.created_at));
    detail_field!(rows, "Updated At", format_date(&template.updated_at));

    print_detail_table(rows);
    Ok(())
}

// ─── Clusters ───

#[derive(Tabled)]
struct ClusterRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Slug")]
    slug: String,
    #[tabled(rename = "Title")]
    title: String,
    #[tabled(rename = "Markets")]
    markets: String,
    #[tabled(rename = "Bond Required")]
    bond_required: String,
    #[tabled(rename = "Status")]
    status: String,
}

fn cluster_to_row(c: &QuestionCluster) -> ClusterRow {
    ClusterRow {
        id: c.id.to_string(),
        slug: truncate(&c.slug, 30),
        title: truncate(&c.title, 50),
        markets: c
            .market_count
            .map(|n| n.to_string())
            .unwrap_or_else(|| DASH.to_string()),
        bond_required: c
            .bond_required
            .map(|b| if b { "Yes" } else { "No" }.to_string())
            .unwrap_or_else(|| DASH.to_string()),
        status: c.status.clone(),
    }
}

pub fn print_clusters(clusters: &[QuestionCluster], output: &OutputFormat) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            if clusters.is_empty() {
                println!("No clusters found.");
                return Ok(());
            }
            let rows: Vec<ClusterRow> = clusters.iter().map(cluster_to_row).collect();
            let table = Table::new(rows).with(Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => print_json(clusters)?,
    }
    Ok(())
}

pub fn print_cluster(cluster: &QuestionCluster, output: &OutputFormat) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(cluster);
    }
    let mut rows: Vec<[String; 2]> = Vec::new();

    detail_field!(rows, "ID", cluster.id.to_string());
    detail_field!(rows, "Template ID", cluster.template_id.to_string());
    detail_field!(rows, "Slug", cluster.slug.clone());
    detail_field!(rows, "Title", cluster.title.clone());
    detail_field!(
        rows,
        "Description",
        cluster.description.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(rows, "Status", cluster.status.clone());
    detail_field!(
        rows,
        "Cluster Constraints",
        cluster
            .cluster_constraints
            .as_ref()
            .map(|v| serde_json::to_string_pretty(v).unwrap_or_default())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Tick Size",
        cluster
            .tick_size
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Fee BPS",
        cluster
            .fee_bps
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Neg Risk",
        cluster
            .neg_risk
            .map(|b| b.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Neg Risk Market ID",
        cluster
            .neg_risk_market_id
            .as_deref()
            .unwrap_or(DASH)
            .to_string()
    );
    detail_field!(
        rows,
        "Market Count",
        cluster
            .market_count
            .map(|n| n.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Bond Required",
        cluster
            .bond_required
            .map(|b| if b { "Yes" } else { "No" }.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Min Bond",
        cluster
            .min_bond
            .map(|d| format_decimal(d))
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Min Fee Rate",
        cluster
            .min_fee_rate
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Max Fee Rate",
        cluster
            .max_fee_rate
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Auction Duration (secs)",
        cluster
            .auction_duration_seconds
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Data",
        cluster
            .data
            .as_ref()
            .map(|v| serde_json::to_string_pretty(v).unwrap_or_default())
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(rows, "Created At", format_date(&cluster.created_at));
    detail_field!(rows, "Updated At", format_date(&cluster.updated_at));

    print_detail_table(rows);
    Ok(())
}

// ─── Create Question ───

pub fn print_create_question(
    response: &CreateQuestionResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(response);
    }
    let mut rows: Vec<[String; 2]> = Vec::new();

    detail_field!(rows, "Condition ID", response.condition_id.clone());
    detail_field!(rows, "Question Text", response.question_text.clone());
    detail_field!(rows, "Outcomes", response.outcomes.join(", "));
    detail_field!(rows, "Token IDs", response.token_ids.join(", "));
    detail_field!(rows, "Status", response.status.clone());
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
        "Cluster Slug",
        response.cluster_slug.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(
        rows,
        "Bond Posted",
        response
            .bond_posted
            .map(|b| if b { "Yes" } else { "No" }.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );

    print_detail_table(rows);
    Ok(())
}

// ─── Resolve Question ───

pub fn print_resolve_question(
    response: &ResolveQuestionResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(response);
    }
    let mut rows: Vec<[String; 2]> = Vec::new();

    detail_field!(rows, "Condition ID", response.condition_id.clone());
    detail_field!(rows, "Status", response.status.clone());
    detail_field!(rows, "Message", response.message.clone());

    print_detail_table(rows);
    Ok(())
}

// ─── Related Markets ───

#[derive(Tabled)]
struct EdgeRow {
    #[tabled(rename = "From")]
    from: String,
    #[tabled(rename = "To")]
    to: String,
    #[tabled(rename = "Edge Type")]
    edge_type: String,
    #[tabled(rename = "Created")]
    created: String,
}

pub fn print_related_markets(
    response: &RelatedMarketsResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            if response.edges.is_empty() {
                println!("No related markets found.");
                return Ok(());
            }
            let rows: Vec<EdgeRow> = response
                .edges
                .iter()
                .map(|e| EdgeRow {
                    from: truncate(&e.from_condition_id, 50),
                    to: truncate(&e.to_condition_id, 50),
                    edge_type: e.edge_type.clone(),
                    created: format_date(&e.created_at),
                })
                .collect();
            let table = Table::new(rows).with(Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => print_json(response)?,
    }
    Ok(())
}

// ─── Market Fees ───

pub fn print_market_fees(
    response: &MarketFeesResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(response);
    }
    let mut rows: Vec<[String; 2]> = Vec::new();

    detail_field!(rows, "Condition ID", response.condition_id.clone());
    detail_field!(
        rows,
        "Creator Agent",
        response
            .creator_agent
            .as_deref()
            .unwrap_or(DASH)
            .to_string()
    );
    detail_field!(
        rows,
        "Creator Fee Rate",
        response
            .creator_fee_rate
            .map(|d| d.to_string())
            .unwrap_or_else(|| DASH.to_string())
    );

    if let Some(ref fees) = response.fee_summary {
        detail_field!(
            rows,
            "Total Taker Fees",
            format_decimal(fees.total_taker_fees)
        );
        detail_field!(
            rows,
            "Total Creator Share",
            format_decimal(fees.total_creator_share)
        );
        detail_field!(
            rows,
            "Total Maker Rebate",
            format_decimal(fees.total_maker_rebate_share)
        );
        detail_field!(
            rows,
            "Total Platform Share",
            format_decimal(fees.total_platform_share)
        );
        detail_field!(rows, "Trade Count", fees.trade_count.to_string());
    } else {
        detail_field!(rows, "Fee Summary", DASH.to_string());
    }

    print_detail_table(rows);
    Ok(())
}
