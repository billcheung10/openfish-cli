use openfish_client_sdk::clob::types::questions::{
    AgentBond, BondRequirementResponse, GetMarketBondResponse, PostBondResponse,
};
use tabled::settings::Style;
use tabled::{Table, Tabled};

use super::{
    DASH, OutputFormat, detail_field, format_date, format_decimal, print_detail_table, print_json,
    truncate,
};

// ─── Bond List ───

#[derive(Tabled)]
struct BondRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Condition ID")]
    condition_id: String,
    #[tabled(rename = "Amount")]
    amount: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Created")]
    created: String,
}

fn bond_to_row(b: &AgentBond) -> BondRow {
    BondRow {
        id: b.id.to_string(),
        condition_id: truncate(&b.condition_id, 50),
        amount: format_decimal(b.amount),
        status: b.status.clone(),
        created: format_date(&b.created_at),
    }
}

pub fn print_bonds(bonds: &[AgentBond], output: &OutputFormat) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            if bonds.is_empty() {
                println!("No bonds found.");
                return Ok(());
            }
            let rows: Vec<BondRow> = bonds.iter().map(bond_to_row).collect();
            let table = Table::new(rows).with(Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => print_json(bonds)?,
    }
    Ok(())
}

// ─── Market Bond Detail ───

#[derive(Tabled)]
struct AuditRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Action")]
    action: String,
    #[tabled(rename = "Amount")]
    amount: String,
    #[tabled(rename = "Reason")]
    reason: String,
    #[tabled(rename = "Created")]
    created: String,
}

fn print_bond_detail(bond: &AgentBond) {
    let mut rows: Vec<[String; 2]> = Vec::new();

    detail_field!(rows, "ID", bond.id.to_string());
    detail_field!(rows, "Agent", bond.agent_address.clone());
    detail_field!(rows, "Condition ID", bond.condition_id.clone());
    detail_field!(rows, "Amount", format_decimal(bond.amount));
    detail_field!(rows, "Status", bond.status.clone());
    detail_field!(
        rows,
        "Slash Reason",
        bond.slash_reason.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(
        rows,
        "Slash Amount",
        bond.slash_amount
            .map(|d| format_decimal(d))
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(rows, "Created At", format_date(&bond.created_at));
    detail_field!(
        rows,
        "Released At",
        bond.released_at
            .as_ref()
            .map(|d| format_date(d))
            .unwrap_or_else(|| DASH.to_string())
    );
    detail_field!(
        rows,
        "Slashed At",
        bond.slashed_at
            .as_ref()
            .map(|d| format_date(d))
            .unwrap_or_else(|| DASH.to_string())
    );

    print_detail_table(rows);
}

pub fn print_market_bond(
    response: &GetMarketBondResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(response);
    }

    if let Some(ref bond) = response.bond {
        print_bond_detail(bond);
    } else {
        println!("No bond found.");
    }

    // Print audit log table
    if !response.audit_log.is_empty() {
        println!("\nAudit Log:");
        let audit_rows: Vec<AuditRow> = response
            .audit_log
            .iter()
            .map(|e| AuditRow {
                id: e.id.to_string(),
                action: e.action.clone(),
                amount: format_decimal(e.amount),
                reason: e
                    .reason
                    .as_deref()
                    .map(|s| truncate(s, 40))
                    .unwrap_or_else(|| DASH.to_string()),
                created: format_date(&e.created_at),
            })
            .collect();
        let table = Table::new(audit_rows).with(Style::rounded()).to_string();
        println!("{table}");
    }

    Ok(())
}

// ─── Post Bond ───

pub fn print_post_bond(response: &PostBondResponse, output: &OutputFormat) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(response);
    }

    print_bond_detail(&response.bond);

    let mut rows: Vec<[String; 2]> = Vec::new();
    detail_field!(rows, "Message", response.message.clone());
    print_detail_table(rows);

    Ok(())
}

// ─── Bond Requirement ───

pub fn print_bond_requirement(
    response: &BondRequirementResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(response);
    }
    let mut rows: Vec<[String; 2]> = Vec::new();

    detail_field!(rows, "Cluster ID", response.cluster_id.to_string());
    detail_field!(
        rows,
        "Bond Required",
        if response.bond_required { "Yes" } else { "No" }.to_string()
    );
    detail_field!(rows, "Min Bond", format_decimal(response.min_bond));

    print_detail_table(rows);
    Ok(())
}
