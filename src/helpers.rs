use alloy::{
    consensus::{Receipt, ReceiptWithBloom, TxReceipt, TxType},
    rpc::types::TransactionReceipt,
};
use anyhow::{Context, Result};

pub fn encode_receipt(receipt: &TransactionReceipt) -> Result<Vec<u8>> {
    let tx_type = receipt.transaction_type();
    let receipt = receipt
        .inner
        .as_receipt_with_bloom()
        .context("Failed to extract inner receipts with bloom")?;
    let logs = receipt
        .logs()
        .iter()
        .map(|l| l.inner.clone())
        .collect::<Vec<_>>();

    let consensus_receipt = Receipt {
        cumulative_gas_used: receipt.cumulative_gas_used(),
        status: receipt.status_or_post_state(),
        logs,
    };

    let rwb = ReceiptWithBloom::new(consensus_receipt, receipt.bloom());
    let encoded = alloy::rlp::encode(rwb);

    match tx_type {
        TxType::Legacy => Ok(encoded),
        _ => Ok([Vec::from([tx_type as u8]), encoded].concat()),
    }
}
