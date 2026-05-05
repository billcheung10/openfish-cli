use openfish_client_sdk::bridge::types::{
    DepositResponse, DepositTransactionStatus, QuoteResponse, StatusResponse,
    SupportedAssetsResponse, SwapExecuteResponse, SwapQuoteResponse, SwapStatusResponse,
    WithdrawPreviewResponse, WithdrawResponse,
};
use serde_json::json;
use tabled::settings::Style;
use tabled::{Table, Tabled};

use super::{DASH, OutputFormat, detail_field, format_decimal, print_detail_table};

pub fn print_deposit(response: &DepositResponse, output: &OutputFormat) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            let mut rows = Vec::new();
            detail_field!(rows, "EVM", format!("{}", response.address.evm));
            detail_field!(
                rows,
                "Solana",
                response
                    .address
                    .svm
                    .clone()
                    .unwrap_or_else(|| DASH.to_owned())
            );
            detail_field!(
                rows,
                "Bitcoin",
                response
                    .address
                    .btc
                    .clone()
                    .unwrap_or_else(|| DASH.to_owned())
            );
            if let Some(note) = &response.note {
                detail_field!(rows, "Note", note.clone());
            }
            print_detail_table(rows);
        }
        OutputFormat::Json => {
            let data = json!({
                "evm": format!("{}", response.address.evm),
                "svm": response.address.svm,
                "btc": response.address.btc,
                "note": response.note,
            });
            super::print_json(&data)?;
        }
    }
    Ok(())
}

pub fn print_supported_assets(
    response: &SupportedAssetsResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            if response.supported_assets.is_empty() {
                println!("No supported assets found.");
                return Ok(());
            }
            #[derive(Tabled)]
            struct Row {
                #[tabled(rename = "Chain")]
                chain: String,
                #[tabled(rename = "Chain ID")]
                chain_id: String,
                #[tabled(rename = "Token")]
                token: String,
                #[tabled(rename = "Symbol")]
                symbol: String,
                #[tabled(rename = "Decimals")]
                decimals: String,
                #[tabled(rename = "Min Deposit")]
                min_deposit: String,
            }
            let rows: Vec<Row> = response
                .supported_assets
                .iter()
                .map(|a| Row {
                    chain: a.chain_name.clone(),
                    chain_id: a.chain_id.to_string(),
                    token: a.token.name.clone(),
                    symbol: a.token.symbol.clone(),
                    decimals: a.token.decimals.to_string(),
                    min_deposit: format_decimal(a.min_checkout_usd),
                })
                .collect();
            let table = Table::new(rows).with(Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => {
            let assets: Vec<_> = response
                .supported_assets
                .iter()
                .map(|a| {
                    json!({
                        "chain_id": a.chain_id,
                        "chain_name": a.chain_name,
                        "token_name": a.token.name,
                        "token_symbol": a.token.symbol,
                        "token_address": a.token.address,
                        "token_decimals": a.token.decimals,
                        "min_checkout_usd": a.min_checkout_usd.to_string(),
                    })
                })
                .collect();
            let data = json!({
                "supported_assets": assets,
                "note": response.note,
            });
            super::print_json(&data)?;
        }
    }
    Ok(())
}

fn format_status(s: &DepositTransactionStatus) -> &'static str {
    match s {
        DepositTransactionStatus::DepositDetected => "Detected",
        DepositTransactionStatus::Processing => "Processing",
        DepositTransactionStatus::OriginTxConfirmed => "Confirmed",
        DepositTransactionStatus::Submitted => "Submitted",
        DepositTransactionStatus::Completed => "Completed",
        DepositTransactionStatus::Failed => "Failed",
        _ => "Unknown",
    }
}

pub fn print_status(response: &StatusResponse, output: &OutputFormat) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            if response.transactions.is_empty()
                && response.deposits.is_empty()
                && response.withdrawals.is_empty()
            {
                println!("No transactions found.");
                return Ok(());
            }
            #[derive(Tabled)]
            struct Row {
                #[tabled(rename = "Type")]
                kind: String,
                #[tabled(rename = "From Chain")]
                from_chain: String,
                #[tabled(rename = "To Chain")]
                to_chain: String,
                #[tabled(rename = "Token")]
                token: String,
                #[tabled(rename = "Amount")]
                amount: String,
                #[tabled(rename = "Status")]
                status: String,
                #[tabled(rename = "Tx Hash")]
                tx_hash: String,
            }
            let rows: Vec<Row> = response
                .transactions
                .iter()
                .chain(response.deposits.iter())
                .map(|tx| Row {
                    kind: "Deposit".to_owned(),
                    from_chain: tx.from_chain_id.to_string(),
                    to_chain: tx.to_chain_id.to_string(),
                    token: super::truncate(&tx.from_token_address, 14),
                    amount: tx.from_amount_base_unit.to_string(),
                    status: format_status(&tx.status).into(),
                    tx_hash: tx
                        .tx_hash
                        .as_deref()
                        .map_or_else(|| DASH.into(), |h| super::truncate(h, 14)),
                })
                .chain(response.withdrawals.iter().map(|tx| {
                    Row {
                        kind: "Withdrawal".to_owned(),
                        from_chain: DASH.to_owned(),
                        to_chain: tx.to_chain_id.to_string(),
                        token: super::truncate(&tx.to_token_address, 14),
                        amount: tx
                            .amount_base_unit
                            .clone()
                            .unwrap_or_else(|| DASH.to_owned()),
                        status: tx.status.clone(),
                        tx_hash: tx
                            .tx_hash
                            .as_deref()
                            .map_or_else(|| DASH.into(), |h| super::truncate(h, 14)),
                    }
                }))
                .collect();
            let table = Table::new(rows).with(Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => {
            let deposits: Vec<_> = response
                .transactions
                .iter()
                .chain(response.deposits.iter())
                .map(|tx| {
                    json!({
                        "from_chain_id": tx.from_chain_id,
                        "from_token_address": tx.from_token_address,
                        "from_amount_base_unit": tx.from_amount_base_unit.to_string(),
                        "to_chain_id": tx.to_chain_id,
                        "to_token_address": format!("{}", tx.to_token_address),
                        "status": format_status(&tx.status),
                        "tx_hash": tx.tx_hash,
                        "created_time_ms": tx.created_time_ms,
                    })
                })
                .collect();
            let withdrawals: Vec<_> = response
                .withdrawals
                .iter()
                .map(|tx| {
                    json!({
                        "to_chain_id": tx.to_chain_id,
                        "to_token_address": tx.to_token_address,
                        "recipient_addr": tx.recipient_addr,
                        "status": tx.status,
                        "tx_hash": tx.tx_hash,
                        "amount_base_unit": tx.amount_base_unit,
                        "fee_base_unit": tx.fee_base_unit,
                        "gas_fee_usd": tx.gas_fee_usd,
                        "created_time_ms": tx.created_time_ms,
                    })
                })
                .collect();
            super::print_json(&json!({
                "deposits": deposits,
                "withdrawals": withdrawals,
            }))?;
        }
    }
    Ok(())
}

pub fn print_withdraw(response: &WithdrawResponse, output: &OutputFormat) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            let mut rows = Vec::new();
            detail_field!(rows, "EVM", format!("{}", response.address.evm));
            detail_field!(
                rows,
                "Solana",
                response
                    .address
                    .svm
                    .clone()
                    .unwrap_or_else(|| DASH.to_owned())
            );
            detail_field!(
                rows,
                "Bitcoin",
                response
                    .address
                    .btc
                    .clone()
                    .unwrap_or_else(|| DASH.to_owned())
            );
            detail_field!(rows, "Note", response.note.clone());
            print_detail_table(rows);
        }
        OutputFormat::Json => {
            let data = json!({
                "evm": format!("{}", response.address.evm),
                "svm": response.address.svm,
                "btc": response.address.btc,
                "note": response.note,
            });
            super::print_json(&data)?;
        }
    }
    Ok(())
}

pub fn print_withdraw_preview(
    response: &WithdrawPreviewResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            let mut rows = Vec::new();
            detail_field!(rows, "Openfish Address", response.address.clone());
            detail_field!(rows, "Chain", response.chain_name.clone());
            detail_field!(rows, "Chain ID", response.to_chain_id.to_string());
            detail_field!(rows, "Token", response.token.symbol.clone());
            detail_field!(rows, "Token Address", response.token.address.clone());
            detail_field!(rows, "Recipient", response.recipient_addr.clone());
            detail_field!(rows, "CLOB Balance", response.clob_balance.clone());
            detail_field!(rows, "Amount Base Unit", response.amount_base_unit.clone());
            detail_field!(rows, "Fee Base Unit", response.fee_base_unit.clone());
            detail_field!(
                rows,
                "Total Debit Base Unit",
                response.total_debit_base_unit.clone()
            );
            detail_field!(
                rows,
                "Hot Wallet Base Unit",
                response.hot_wallet_balance_base_unit.clone()
            );
            detail_field!(rows, "Gas BNB Wei", response.gas_bnb_wei.clone());
            detail_field!(rows, "Fee Source", response.fee_source.clone());
            detail_field!(rows, "Can Withdraw", response.can_withdraw.to_string());
            print_detail_table(rows);
        }
        OutputFormat::Json => {
            let data = json!({
                "address": response.address,
                "toChainId": response.to_chain_id,
                "chainName": response.chain_name,
                "token": {
                    "name": response.token.name,
                    "symbol": response.token.symbol,
                    "address": response.token.address,
                    "decimals": response.token.decimals,
                },
                "recipientAddr": response.recipient_addr,
                "clobBalance": response.clob_balance,
                "clobBalanceBaseUnit": response.clob_balance_base_unit,
                "amountBaseUnit": response.amount_base_unit,
                "feeBaseUnit": response.fee_base_unit,
                "totalDebitBaseUnit": response.total_debit_base_unit,
                "hotWalletBalanceBaseUnit": response.hot_wallet_balance_base_unit,
                "gasBnbWei": response.gas_bnb_wei,
                "feeSource": response.fee_source,
                "canWithdraw": response.can_withdraw,
            });
            super::print_json(&data)?;
        }
    }
    Ok(())
}

pub fn print_quote(response: &QuoteResponse, output: &OutputFormat) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            let mut rows = Vec::new();
            detail_field!(rows, "Quote ID", response.quote_id.clone());
            detail_field!(
                rows,
                "Est. Input",
                format!("${:.2}", response.est_input_usd)
            );
            detail_field!(
                rows,
                "Est. Output",
                format!("${:.2}", response.est_output_usd)
            );
            detail_field!(
                rows,
                "Output Tokens",
                response.est_to_token_base_unit.to_string()
            );
            detail_field!(
                rows,
                "Est. Time",
                format!("{}ms", response.est_checkout_time_ms)
            );

            let fee = &response.est_fee_breakdown;
            detail_field!(rows, "App Fee", format!("{:.2}%", fee.app_fee_percent));
            detail_field!(rows, "Gas", format!("${:.4}", fee.gas_usd));
            detail_field!(
                rows,
                "Total Impact",
                format!("{:.2}% (${:.4})", fee.total_impact, fee.total_impact_usd)
            );
            detail_field!(rows, "Min Received", format!("${:.4}", fee.min_received));
            print_detail_table(rows);
        }
        OutputFormat::Json => {
            let data = json!({
                "quote_id": response.quote_id,
                "est_input_usd": response.est_input_usd,
                "est_output_usd": response.est_output_usd,
                "est_to_token_base_unit": response.est_to_token_base_unit.to_string(),
                "est_checkout_time_ms": response.est_checkout_time_ms,
                "est_fee_breakdown": {
                    "app_fee_label": response.est_fee_breakdown.app_fee_label,
                    "app_fee_percent": response.est_fee_breakdown.app_fee_percent,
                    "app_fee_usd": response.est_fee_breakdown.app_fee_usd,
                    "fill_cost_percent": response.est_fee_breakdown.fill_cost_percent,
                    "fill_cost_usd": response.est_fee_breakdown.fill_cost_usd,
                    "gas_usd": response.est_fee_breakdown.gas_usd,
                    "max_slippage": response.est_fee_breakdown.max_slippage,
                    "min_received": response.est_fee_breakdown.min_received,
                    "swap_impact": response.est_fee_breakdown.swap_impact,
                    "swap_impact_usd": response.est_fee_breakdown.swap_impact_usd,
                    "total_impact": response.est_fee_breakdown.total_impact,
                    "total_impact_usd": response.est_fee_breakdown.total_impact_usd,
                },
            });
            super::print_json(&data)?;
        }
    }
    Ok(())
}

pub fn print_swap_quote(response: &SwapQuoteResponse, output: &OutputFormat) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            let mut rows = Vec::new();
            detail_field!(rows, "Quote ID", response.quote_id.clone());
            detail_field!(rows, "Openfish Address", response.address.clone());
            detail_field!(rows, "Deposit Address", response.deposit_address.clone());
            detail_field!(rows, "From", response.from_token.clone());
            detail_field!(rows, "To Token", response.to_token_address.clone());
            detail_field!(rows, "Amount In Wei", response.amount_in_wei.clone());
            detail_field!(
                rows,
                "Estimated FISH",
                response.estimated_amount_out_base_unit.clone()
            );
            detail_field!(
                rows,
                "Minimum FISH",
                response.min_amount_out_base_unit.clone()
            );
            detail_field!(rows, "Slippage Bps", response.slippage_bps.to_string());
            detail_field!(
                rows,
                "Price Impact Bps",
                response
                    .price_impact_bps
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| DASH.to_owned())
            );
            detail_field!(rows, "Required BNB Wei", response.required_bnb_wei.clone());
            detail_field!(
                rows,
                "Deposit BNB Wei",
                response.deposit_bnb_balance_wei.clone()
            );
            detail_field!(rows, "Has Enough BNB", response.has_enough_bnb.to_string());
            detail_field!(rows, "Expires At", response.expires_at.clone());
            print_detail_table(rows);
        }
        OutputFormat::Json => {
            super::print_json(&json!(response))?;
        }
    }
    Ok(())
}

pub fn print_swap_execute(
    response: &SwapExecuteResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            let mut rows = Vec::new();
            detail_field!(rows, "Swap ID", response.swap_id.clone());
            detail_field!(rows, "Quote ID", response.quote_id.clone());
            detail_field!(rows, "Status", response.status.clone());
            detail_field!(rows, "Swap Tx", response.swap_tx_hash.clone());
            detail_field!(
                rows,
                "Sweep Tx",
                response
                    .sweep_tx_hash
                    .clone()
                    .unwrap_or_else(|| DASH.to_owned())
            );
            detail_field!(
                rows,
                "FISH Received",
                response
                    .fish_received_base_unit
                    .clone()
                    .unwrap_or_else(|| DASH.to_owned())
            );
            detail_field!(
                rows,
                "FISH Credited",
                response
                    .fish_credited_base_unit
                    .clone()
                    .unwrap_or_else(|| DASH.to_owned())
            );
            print_detail_table(rows);
        }
        OutputFormat::Json => {
            super::print_json(&json!(response))?;
        }
    }
    Ok(())
}

pub fn print_swap_status(
    response: &SwapStatusResponse,
    output: &OutputFormat,
) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            let mut rows = Vec::new();
            detail_field!(rows, "Swap ID", response.swap_id.clone());
            detail_field!(rows, "Quote ID", response.quote_id.clone());
            detail_field!(rows, "Openfish Address", response.address.clone());
            detail_field!(rows, "Deposit Address", response.deposit_address.clone());
            detail_field!(rows, "Status", response.status.clone());
            detail_field!(rows, "Amount In Wei", response.amount_in_wei.clone());
            detail_field!(
                rows,
                "Minimum FISH",
                response.min_amount_out_base_unit.clone()
            );
            detail_field!(
                rows,
                "FISH Received",
                response
                    .fish_received_base_unit
                    .clone()
                    .unwrap_or_else(|| DASH.to_owned())
            );
            detail_field!(
                rows,
                "FISH Credited",
                response
                    .fish_credited_base_unit
                    .clone()
                    .unwrap_or_else(|| DASH.to_owned())
            );
            detail_field!(
                rows,
                "Swap Tx",
                response
                    .swap_tx_hash
                    .clone()
                    .unwrap_or_else(|| DASH.to_owned())
            );
            detail_field!(
                rows,
                "Sweep Tx",
                response
                    .sweep_tx_hash
                    .clone()
                    .unwrap_or_else(|| DASH.to_owned())
            );
            detail_field!(
                rows,
                "Error",
                response.error.clone().unwrap_or_else(|| DASH.to_owned())
            );
            detail_field!(rows, "Created At", response.created_at.clone());
            detail_field!(rows, "Updated At", response.updated_at.clone());
            print_detail_table(rows);
        }
        OutputFormat::Json => {
            super::print_json(&json!(response))?;
        }
    }
    Ok(())
}

pub fn print_swap_list(
    response: &[SwapStatusResponse],
    output: &OutputFormat,
) -> anyhow::Result<()> {
    match output {
        OutputFormat::Table => {
            if response.is_empty() {
                println!("No swaps found.");
                return Ok(());
            }
            #[derive(Tabled)]
            struct Row {
                #[tabled(rename = "Swap ID")]
                swap_id: String,
                #[tabled(rename = "Status")]
                status: String,
                #[tabled(rename = "Amount In Wei")]
                amount_in_wei: String,
                #[tabled(rename = "FISH Credited")]
                fish_credited: String,
                #[tabled(rename = "Swap Tx")]
                swap_tx: String,
                #[tabled(rename = "Updated")]
                updated: String,
            }
            let rows: Vec<Row> = response
                .iter()
                .map(|s| Row {
                    swap_id: super::truncate(&s.swap_id, 14),
                    status: s.status.clone(),
                    amount_in_wei: s.amount_in_wei.clone(),
                    fish_credited: s
                        .fish_credited_base_unit
                        .clone()
                        .unwrap_or_else(|| DASH.to_owned()),
                    swap_tx: s
                        .swap_tx_hash
                        .as_deref()
                        .map_or_else(|| DASH.to_owned(), |h| super::truncate(h, 14)),
                    updated: s.updated_at.clone(),
                })
                .collect();
            let table = Table::new(rows).with(Style::rounded()).to_string();
            println!("{table}");
        }
        OutputFormat::Json => {
            super::print_json(response)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_status_all_variants() {
        assert_eq!(
            format_status(&DepositTransactionStatus::DepositDetected),
            "Detected"
        );
        assert_eq!(
            format_status(&DepositTransactionStatus::Processing),
            "Processing"
        );
        assert_eq!(
            format_status(&DepositTransactionStatus::OriginTxConfirmed),
            "Confirmed"
        );
        assert_eq!(
            format_status(&DepositTransactionStatus::Submitted),
            "Submitted"
        );
        assert_eq!(
            format_status(&DepositTransactionStatus::Completed),
            "Completed"
        );
        assert_eq!(format_status(&DepositTransactionStatus::Failed), "Failed");
    }
}
