use soroban_sdk::{contracttype, Address, Bytes, BytesN, Vec};

#[derive(Clone, Debug, PartialEq)]
#[contracttype]
pub enum OracleType {
    EdnaLab = 0,
    CameraTrapAi = 1,
    SatelliteImagery = 2,
    FieldSurvey = 3,
    AcousticSensor = 4,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct OracleNode {
    pub pubkey: BytesN<32>,
    pub uri: Bytes,
    pub oracle_type: OracleType,
    pub active: bool,
    pub registered_at: u64,
    pub total_surveys: u64,
    pub accuracy_score: u32,  // 0-100
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
pub struct HabitatPolygon {
    pub polygon_id: BytesN<32>,
    pub geometry_ipfs_cid: Bytes,
    pub bounding_box: BoundingBox,
    pub area_ha: u64,
    pub biome: u32,
    pub country: BytesN<2>,
    pub project_id: BytesN<32>,
    pub registered_at: u64,
    pub active: bool,
    pub total_credits_minted: u64,
    pub total_credits_retired: u64,
    pub last_survey_cid: Option<Bytes>,
    pub last_survey_timestamp: Option<u64>,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct SurveyRecord {
    pub survey_hash: BytesN<32>,
    pub polygon_id: BytesN<32>,
    pub ipfs_cid: Bytes,
    pub survey_timestamp: u64,
    pub oracle_count: u32,
    pub threshold_met: bool,
    pub disputed: bool,
    pub resolved: bool,
    pub token_ids: Vec<u64>,
    pub analyses_hashes: Vec<BytesN<32>>,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct SurveyData {
    pub polygon_id: BytesN<32>,
    pub ipfs_cid: Bytes,
    pub survey_timestamp: u64,
    pub signatures: Vec<(BytesN<32>, BytesN<64>)>,
    pub analyses_hashes: Vec<BytesN<32>>,
    pub baseline_bsi: u32,
    pub current_bsi: u32,
    pub area_contribution: u64,
    pub biome: u32,
    pub vintage_year: u32,
    pub vintage_qtr: u32,
    pub methodology_id: BytesN<8>,
    pub beneficiary: Address,
}
