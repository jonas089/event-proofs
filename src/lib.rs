mod helpers;
mod types;

use alloy::{
    consensus::ReceiptWithBloom,
    hex,
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionReceipt,
};
use alloy_primitives::FixedBytes;
use alloy_rlp::{self};
use alloy_sol_types::{SolType, sol};
use alloy_trie::proof::verify_proof;
use alloy_trie::{HashBuilder, Nibbles, proof::ProofRetainer, root::adjust_index_for_rlp};
use anyhow::{Context, Ok, Result};
use std::str::FromStr;
use url::Url;

use crate::{helpers::encode_receipt, types::EthereumReceiptProof};
use types::SimpleEvent as SimpleEventRs;

sol! {
    struct SimpleEvent {
        uint256 amount;
        address account;
    }
}

pub struct MerkleProver {
    pub provider: String,
}

impl MerkleProver {
    pub fn new(provider: String) -> Self {
        Self { provider }
    }

    pub async fn get_proof(&self, index: usize, height: u64) -> Result<EthereumReceiptProof> {
        let provider = ProviderBuilder::new().on_http(Url::from_str(&self.provider)?);
        let receipts: Vec<TransactionReceipt> = provider
            .get_block_receipts(alloy::eips::BlockId::Number(
                alloy::eips::BlockNumberOrTag::Number(height),
            ))
            .await?
            .context("Failed to get block receipts")?;
        let retainer =
            ProofRetainer::new(vec![Nibbles::unpack(alloy_rlp::encode_fixed_size(&index))]);
        let mut hb: HashBuilder = HashBuilder::default().with_proof_retainer(retainer);
        for i in 0..receipts.len() {
            let index = adjust_index_for_rlp(i, receipts.len());
            let index_buffer = alloy_rlp::encode_fixed_size(&index);
            hb.add_leaf(
                Nibbles::unpack(&index_buffer),
                encode_receipt(&receipts[index])?.as_slice(),
            );
        }
        let receipt_key: Vec<u8> = alloy_rlp::encode(index);
        hb.root();
        let proof = hb
            .take_proof_nodes()
            .into_nodes_sorted()
            .into_iter()
            .map(|n| n.1)
            .collect::<Vec<_>>()
            .iter()
            .map(|n| n.to_vec())
            .collect::<Vec<_>>();

        let leaf_node_decoded: Vec<alloy_rlp::Bytes> = alloy_rlp::decode_exact(
            proof
                .to_vec()
                .last()
                .context("Failed to extract leaf from receipt proof")?,
        )?;
        let receipt_rlp = leaf_node_decoded
            .last()
            .context("Failed to extract value from leaf")?
            .to_vec();
        Ok(EthereumReceiptProof {
            proof,
            key: receipt_key,
            value: receipt_rlp,
        })
    }
}

impl EthereumReceiptProof {
    pub fn new(proof: Vec<Vec<u8>>, key: Vec<u8>, value: Vec<u8>) -> EthereumReceiptProof {
        EthereumReceiptProof { proof, key, value }
    }
    pub fn verify(&self, root: &[u8], index: usize) -> Result<bool> {
        let proof_nodes: Vec<alloy_primitives::Bytes> = self
            .proof
            .iter()
            .map(|node| alloy_primitives::Bytes::copy_from_slice(node))
            .collect();
        let key = Nibbles::unpack(&self.key);
        let result = verify_proof(
            FixedBytes::from_slice(&root),
            key,
            Some(self.value.to_vec()),
            proof_nodes.iter(),
        );

        // decode the ReceiptWithBloom
        let receipt: ReceiptWithBloom = alloy_rlp::decode_exact(&self.value).unwrap();

        // decode raw event
        let event_decoded: SimpleEvent =
            SimpleEvent::abi_decode(&receipt.receipt.logs.get(index).unwrap().data.data).unwrap();

        let event_topis = receipt.receipt.logs.get(index).unwrap().data.topics();

        println!(
            "Event decoded: {:?}, topic (fist only): {:?}",
            &SimpleEventRs {
                amount: *event_decoded.amount.as_limbs().first().unwrap(),
                address: event_decoded.account.to_string()
            },
            hex::encode(event_topis.first().unwrap())
        );

        match result {
            core::result::Result::Ok(_) => Ok(true),
            Err(e) => {
                anyhow::bail!("Proof verification failed: {:?}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::MerkleProver;
    use alloy::hex;

    #[tokio::test]
    async fn test_verify_receipt() {
        let merkle_prover = MerkleProver::new("http://127.0.0.1:8545".to_string());

        // index 0 if you want to test just one (the first) event, use block number from start.sh output for testing as "height" and get the corresponding root for verification
        let height = 2;
        let index = 0;
        let receipts_proof = merkle_prover.get_proof(index, height).await.unwrap();
        let verification_root =
            "0x143a3e32d78668040530ce3420016e16c5f239047bd5ad67c573e5f7a0dbb821";
        receipts_proof
            .verify(&hex::decode(verification_root).unwrap(), index)
            .expect("Failed to verify");

        // Decode as (uint256, address)
    }
}

/* Get the receiptsRoot for a given block number

curl -s -X POST http://127.0.0.1:8545   -H "Content-Type: application/json"   -d '{
    "jsonrpc":"2.0",
    "method":"eth_getBlockByNumber",
    "params":["0x2", false],
    "id":1
  }' | jq -r '.result.receiptsRoot'
*/
