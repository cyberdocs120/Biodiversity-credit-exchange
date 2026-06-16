use soroban_sdk::{contracttype, Address, Bytes, BytesN, Vec};

#[derive(Clone, Debug)]
#[contracttype]
pub enum Biome {
    TropicalForest = 0,
    TemperateForest = 1,
    Grassland = 2,
    Wetland = 3,
    Mangrove = 4,
    CoralReef = 5,
    Other = 6,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct MintParams {
    pub polygon_id: BytesN<32>,
    pub methodology_id: BytesN<8>,
    pub survey_ipfs_cid: Bytes,
    pub baseline_bsi: u32,
    pub current_bsi: u32,
    pub area_ha_contribution: u64,
    pub biome: Biome,
    pub vintage_year: u32,
    pub vintage_quarter: u32,
    pub approval_governance_id: Address,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct ProposeParams {
    pub polygon_id: BytesN<32>,
    pub survey_hash: BytesN<32>,
    pub methodology_id: BytesN<8>,
    pub credit_qty: u64,
    pub beneficiary: Address,
    pub survey_ipfs_cid: Bytes,
    pub baseline_bsi: u32,
    pub current_bsi: u32,
    pub area_ha_contribution: u64,
    pub biome: u32,
    pub vintage_year: u32,
    pub vintage_quarter: u32,
    pub approval_governance_id: Address,
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum StakeholderRole {
    LeadEcologist = 0,
    PeerEcologist = 1,
    LocalCommunityRep = 2,
    IndependentAuditor = 3,
    MethodologyExpert = 4,
    RegulatoryObserver = 5,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Stakeholder {
    pub addr: Address,
    pub role: StakeholderRole,
    pub weight: u32,
    pub has_veto: bool,
    pub active: bool,
    pub registered_at: u64,
}

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum ProposalState {
    Draft = 0,
    Voting = 1,
    Approved = 2,
    Rejected = 3,
    Cancelled = 4,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Vote {
    pub voter: Address,
    pub approve: bool,
    pub weight: u32,
    pub comment_hash: BytesN<32>,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Proposal {
    pub proposal_id: u64,
    pub polygon_id: BytesN<32>,
    pub survey_hash: BytesN<32>,
    pub methodology_id: BytesN<8>,
    pub credit_qty: u64,
    pub beneficiary: Address,
    pub proposer: Address,
    pub created_at: u64,
    pub voting_deadline: u64,
    pub state: ProposalState,
    pub votes: Vec<Vote>,
    pub community_veto: bool,
    pub weighted_total_approve: u32,
    pub weighted_total_reject: u32,
    pub survey_ipfs_cid: Bytes,
    pub baseline_bsi: u32,
    pub current_bsi: u32,
    pub area_ha_contribution: u64,
    pub biome: u32,
    pub vintage_year: u32,
    pub vintage_quarter: u32,
    pub approval_governance_id: Address,
}
