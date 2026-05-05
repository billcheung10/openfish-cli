use openfish_client_sdk::clob::types::questions::DisputeResponse;

use super::{DASH, OutputFormat, detail_field, print_detail_table, print_json};

pub fn print_dispute_result(
    response: &DisputeResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    if matches!(output, OutputFormat::Json) {
        return print_json(response);
    }
    let mut rows: Vec<[String; 2]> = Vec::new();

    detail_field!(rows, "Status", response.status.clone());
    detail_field!(
        rows,
        "TX Hash",
        response.tx_hash.as_deref().unwrap_or(DASH).to_string()
    );
    detail_field!(
        rows,
        "Market Status",
        response
            .market_status
            .as_deref()
            .unwrap_or(DASH)
            .to_string()
    );
    detail_field!(rows, "Message", response.message.clone());

    print_detail_table(rows);
    Ok(())
}
