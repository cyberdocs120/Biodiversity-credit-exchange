use soroban_sdk::{contracttype, Address, Bytes, BytesN};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[contracttype]
pub enum OrderSide { 
    Buy = 0, 
    Sell = 1 
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[contracttype]
pub enum OrderRestriction { 
    None = 0, 
    FillOrKill = 1, 
    ImmediateOrCancel = 2 
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[contracttype]
pub enum OrderStatus { 
    Open = 0, 
    Filled = 1, 
    Cancelled = 2 
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct Order {
    pub order_id: u64,
    pub trader: Address,
    pub side: OrderSide,
    pub price: i128,
    pub initial_qty: u64,
    pub remaining_qty: u64,
    pub timestamp: u64,
    pub restrictions: OrderRestriction,
    pub biome_filter: Option<u32>,
    pub vintage_filter: Option<u32>,
    pub status: OrderStatus,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[contracttype]
pub enum BdcState {
    Active = 0,
    Retired = 1,
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
    pub approval_governance_id: Address,
    pub metadata_uri: Bytes,
    pub state: BdcState,
    pub retired_at: Option<u64>,
    pub retirement_receipt: Option<BytesN<32>>,
}
