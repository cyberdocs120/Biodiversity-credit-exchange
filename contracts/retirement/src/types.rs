use soroban_sdk::{contracttype, Address, Bytes, BytesN, Vec};

#[derive(Clone, Debug)]
#[contracttype]
pub struct ClaimData {
    pub period_start: u64,
    pub period_end: u64,
    pub purpose: Bytes,
    pub jurisdiction: Bytes,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct RetirementReceipt {
    pub receipt_id: BytesN<32>,
    pub retirer: Address,
    pub token_ids: Vec<u64>,
    pub polygon_id: BytesN<32>,
    pub total_credits: u64,
    pub claim_period_start: u64,
    pub claim_period_end: u64,
    pub purpose: Bytes,
    pub jurisdiction: Bytes,
    pub merkle_root: BytesN<32>,
    pub timestamp: u64,
    pub block_height: u64,
}
