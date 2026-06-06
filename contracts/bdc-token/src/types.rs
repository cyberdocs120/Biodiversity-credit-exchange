use soroban_sdk::{contracttype, Address, Bytes, BytesN};

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum BdcState {
    Active = 0,
    Retired = 1,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct BoundingBox {
    pub min_lat: i64,
    pub max_lat: i64,
    pub min_lon: i64,
    pub max_lon: i64,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct BdcMetadata {
    pub token_id: u64,
    pub polygon_id: BytesN<32>,
    pub methodology_id: BytesN<8>,
    pub survey_ipfs_cid: Bytes,
    pub baseline_bsi: u32,
    pub current_bsi: u32,
    pub area_ha_contribution: u64,
    pub biome: Biome,
    pub vintage_year: u32,
    pub vintage_quarter: u32,
    pub approval_governance_id: BytesN<32>,
    pub state: BdcState,
    pub retired_at: Option<u64>,
    pub retirement_receipt: Option<BytesN<32>>,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct BdcTokenValue {
    pub owner: Address,
    pub metadata: BdcMetadata,
}
