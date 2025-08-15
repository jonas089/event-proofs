#[derive(Debug)]
pub struct SimpleEvent {
    pub amount: u64,
    pub address: String,
}

#[derive(Debug, Clone)]
pub struct EthereumReceiptProof {
    /// The list of proof nodes in the Merkle path from leaf to root
    pub proof: Vec<Vec<u8>>,
    /// The original key before hashing (typically the transaction index)
    pub key: Vec<u8>,
    /// The RLP-encoded receipt data being proven
    pub value: Vec<u8>,
}
