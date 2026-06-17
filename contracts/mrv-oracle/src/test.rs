#![cfg(test)]
#![allow(clippy::bool_assert_comparison)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env, Vec};

fn setup_test(env: &Env) -> (Address, MrvOracleContractClient<'static>) {
    let admin = Address::generate(env);
    let contract_id = env.register(MrvOracleContract, ());
    let client = MrvOracleContractClient::new(env, &contract_id);
    client.initialize(&admin);
    (admin, client)
}

fn make_survey_data(
    _env: &Env,
    polygon_id: &BytesN<32>,
    ipfs_cid: &Bytes,
    survey_timestamp: u64,
    signatures: &Vec<(BytesN<32>, BytesN<64>)>,
    analyses_hashes: &Vec<BytesN<32>>,
    baseline_bsi: u32,
    current_bsi: u32,
    area_contribution: u64,
    biome: u32,
    vintage_year: u32,
    vintage_qtr: u32,
    methodology_id: &BytesN<8>,
    beneficiary: &Address,
) -> SurveyData {
    SurveyData {
        polygon_id: polygon_id.clone(),
        ipfs_cid: ipfs_cid.clone(),
        survey_timestamp,
        signatures: signatures.clone(),
        analyses_hashes: analyses_hashes.clone(),
        baseline_bsi,
        current_bsi,
        area_contribution,
        biome,
        vintage_year,
        vintage_qtr,
        methodology_id: methodology_id.clone(),
        beneficiary: beneficiary.clone(),
    }
}

#[test]
fn test_register_oracle() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);

    let pubkey = BytesN::from_array(&env, &[1; 32]);
    let uri = Bytes::from_slice(&env, b"ipfs://oracle1");
    let oracle_type = OracleType::EdnaLab;

    env.mock_all_auths();
    client.register_oracle(&pubkey, &uri, &oracle_type);

    assert_eq!(client.oracle_count(), 1);
    let oracle = client.get_oracle(&pubkey);
    assert_eq!(oracle.active, true);
    assert_eq!(oracle.oracle_type, oracle_type);
}

#[test]
fn test_revoke_oracle() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);

    let pubkey = BytesN::from_array(&env, &[1; 32]);
    let uri = Bytes::from_slice(&env, b"ipfs://oracle1");
    let oracle_type = OracleType::EdnaLab;

    env.mock_all_auths();
    client.register_oracle(&pubkey, &uri, &oracle_type);
    client.revoke_oracle(&pubkey);

    let oracle = client.get_oracle(&pubkey);
    assert_eq!(oracle.active, false);
}

#[test]
fn test_set_threshold() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);

    env.mock_all_auths();
    client.set_threshold(&3, &5);

    let (n, d) = client.threshold();
    assert_eq!(n, 3);
    assert_eq!(d, 5);
}

#[test]
fn test_pause_resume() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);

    env.mock_all_auths();
    assert_eq!(client.paused(), false);

    client.pause();
    assert_eq!(client.paused(), true);

    client.resume();
    assert_eq!(client.paused(), false);
}

#[test]
fn test_register_polygon() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);

    let polygon_id = BytesN::from_array(&env, &[2; 32]);
    let geometry_cid = Bytes::from_slice(&env, b"ipfs://poly1");
    let bbox = BoundingBox {
        min_lat: -10,
        max_lat: 10,
        min_lon: -20,
        max_lon: 20,
    };
    let area_ha = 1000;
    let biome = 1;
    let country = BytesN::from_array(&env, b"BR");
    let project_id = BytesN::from_array(&env, &[3; 32]);

    env.mock_all_auths();
    client.register_polygon(
        &polygon_id,
        &geometry_cid,
        &bbox,
        &area_ha,
        &biome,
        &country,
        &project_id,
    );

    let polygon = client.get_polygon(&polygon_id);
    assert_eq!(polygon.active, true);
    assert_eq!(polygon.area_ha, area_ha);
    assert_eq!(polygon.biome, biome);
}

#[test]
fn test_close_polygon() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);

    let polygon_id = BytesN::from_array(&env, &[2; 32]);
    let geometry_cid = Bytes::from_slice(&env, b"ipfs://poly1");
    let bbox = BoundingBox {
        min_lat: -10,
        max_lat: 10,
        min_lon: -20,
        max_lon: 20,
    };

    env.mock_all_auths();
    client.register_polygon(
        &polygon_id,
        &geometry_cid,
        &bbox,
        &1000,
        &1,
        &BytesN::from_array(&env, b"BR"),
        &BytesN::from_array(&env, &[3; 32]),
    );
    client.close_polygon(&polygon_id);

    let polygon = client.get_polygon(&polygon_id);
    assert_eq!(polygon.active, false);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_duplicate_oracle_rejected() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);

    let pubkey = BytesN::from_array(&env, &[1; 32]);
    let uri = Bytes::from_slice(&env, b"ipfs://oracle1");

    env.mock_all_auths();
    client.register_oracle(&pubkey, &uri, &OracleType::EdnaLab);
    client.register_oracle(&pubkey, &uri, &OracleType::EdnaLab);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_get_nonexistent_oracle() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);

    let pubkey = BytesN::from_array(&env, &[1; 32]);
    client.get_oracle(&pubkey);
}

#[test]
fn test_submit_survey_basic() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);

    let oracle_pubkey = BytesN::from_array(&env, &[1; 32]);
    let uri = Bytes::from_slice(&env, b"ipfs://oracle1");
    env.mock_all_auths();
    client.register_oracle(&oracle_pubkey, &uri, &OracleType::EdnaLab);

    let polygon_id = BytesN::from_array(&env, &[2; 32]);
    let bbox = BoundingBox {
        min_lat: 0,
        max_lat: 10,
        min_lon: 0,
        max_lon: 10,
    };
    client.register_polygon(
        &polygon_id,
        &uri,
        &bbox,
        &100,
        &1,
        &BytesN::from_array(&env, b"US"),
        &BytesN::from_array(&env, &[3; 32]),
    );

    let signatures = Vec::from_array(
        &env,
        [(oracle_pubkey.clone(), BytesN::from_array(&env, &[0; 64]))],
    );
    let analyses = Vec::new(&env);
    let beneficiary = Address::generate(&env);
    let meth_id = BytesN::from_array(&env, &[0; 8]);

    let data = make_survey_data(
        &env,
        &polygon_id,
        &uri,
        1000,
        &signatures,
        &analyses,
        20,
        50,
        100,
        1,
        2024,
        1,
        &meth_id,
        &beneficiary,
    );

    let _survey_hash = client.submit_survey(&data);

    let polygon = client.get_polygon(&polygon_id);
    assert_eq!(polygon.last_survey_cid.unwrap(), uri);
    assert_eq!(polygon.last_survey_timestamp.unwrap(), 1000);
}

#[test]
fn test_dispute_flow() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);

    let oracle_pubkey = BytesN::from_array(&env, &[1; 32]);
    let uri = Bytes::from_slice(&env, b"ipfs://oracle1");
    env.mock_all_auths();
    client.register_oracle(&oracle_pubkey, &uri, &OracleType::EdnaLab);

    let polygon_id = BytesN::from_array(&env, &[2; 32]);
    let bbox = BoundingBox {
        min_lat: 0,
        max_lat: 10,
        min_lon: 0,
        max_lon: 10,
    };
    client.register_polygon(
        &polygon_id,
        &uri,
        &bbox,
        &100,
        &1,
        &BytesN::from_array(&env, b"US"),
        &BytesN::from_array(&env, &[3; 32]),
    );

    let signatures = Vec::from_array(
        &env,
        [(oracle_pubkey.clone(), BytesN::from_array(&env, &[0; 64]))],
    );
    let analyses = Vec::new(&env);
    let beneficiary = Address::generate(&env);
    let meth_id = BytesN::from_array(&env, &[0; 8]);

    let data = make_survey_data(
        &env,
        &polygon_id,
        &uri,
        1000,
        &signatures,
        &analyses,
        20,
        50,
        100,
        1,
        2024,
        1,
        &meth_id,
        &beneficiary,
    );

    let hash = client.submit_survey(&data);

    client.dispute(&hash);

    let slashed = Vec::from_array(&env, [oracle_pubkey.clone()]);
    client.resolve_dispute(&hash, &true, &slashed);

    let oracle = client.get_oracle(&oracle_pubkey);
    assert_eq!(oracle.active, false);
    assert_eq!(oracle.accuracy_score, 0);
}

#[test]
#[should_panic(expected = "Error(Contract, #12)")]
fn test_submit_while_paused() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);
    env.mock_all_auths();
    client.pause();

    let polygon_id = BytesN::from_array(&env, &[2; 32]);
    let uri = Bytes::from_slice(&env, b"ipfs://poly1");
    let signatures = Vec::new(&env);
    let analyses = Vec::new(&env);
    let beneficiary = Address::generate(&env);
    let meth_id = BytesN::from_array(&env, &[0; 8]);

    let data = make_survey_data(
        &env,
        &polygon_id,
        &uri,
        1000,
        &signatures,
        &analyses,
        20,
        50,
        100,
        1,
        2024,
        1,
        &meth_id,
        &beneficiary,
    );

    client.submit_survey(&data);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_submit_below_threshold_rejected() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);
    env.mock_all_auths();
    client.set_threshold(&2, &2);

    let oracle_pubkey = BytesN::from_array(&env, &[1; 32]);
    client.register_oracle(
        &oracle_pubkey,
        &Bytes::from_slice(&env, b"uri"),
        &OracleType::EdnaLab,
    );

    let polygon_id = BytesN::from_array(&env, &[2; 32]);
    let bbox = BoundingBox {
        min_lat: 0,
        max_lat: 10,
        min_lon: 0,
        max_lon: 10,
    };
    client.register_polygon(
        &polygon_id,
        &Bytes::from_slice(&env, b"uri"),
        &bbox,
        &100,
        &1,
        &BytesN::from_array(&env, b"US"),
        &BytesN::from_array(&env, &[3; 32]),
    );

    let signatures = Vec::from_array(
        &env,
        [(oracle_pubkey.clone(), BytesN::from_array(&env, &[0; 64]))],
    );
    let analyses = Vec::new(&env);
    let beneficiary = Address::generate(&env);
    let meth_id = BytesN::from_array(&env, &[0; 8]);

    let data = make_survey_data(
        &env,
        &polygon_id,
        &Bytes::from_slice(&env, b"uri"),
        1000,
        &signatures,
        &analyses,
        20,
        50,
        100,
        1,
        2024,
        1,
        &meth_id,
        &beneficiary,
    );

    client.submit_survey(&data);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_submit_invalid_oracle_rejected() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);
    env.mock_all_auths();

    let polygon_id = BytesN::from_array(&env, &[2; 32]);
    let bbox = BoundingBox {
        min_lat: 0,
        max_lat: 10,
        min_lon: 0,
        max_lon: 10,
    };
    client.register_polygon(
        &polygon_id,
        &Bytes::from_slice(&env, b"uri"),
        &bbox,
        &100,
        &1,
        &BytesN::from_array(&env, b"US"),
        &BytesN::from_array(&env, &[3; 32]),
    );

    let oracle_pubkey = BytesN::from_array(&env, &[1; 32]); // Not registered
    let signatures = Vec::from_array(
        &env,
        [(oracle_pubkey.clone(), BytesN::from_array(&env, &[0; 64]))],
    );
    let analyses = Vec::new(&env);
    let beneficiary = Address::generate(&env);
    let meth_id = BytesN::from_array(&env, &[0; 8]);

    let data = make_survey_data(
        &env,
        &polygon_id,
        &Bytes::from_slice(&env, b"uri"),
        1000,
        &signatures,
        &analyses,
        20,
        50,
        100,
        1,
        2024,
        1,
        &meth_id,
        &beneficiary,
    );

    client.submit_survey(&data);
}

#[test]
#[should_panic(expected = "Error(Contract, #11)")]
fn test_duplicate_survey_rejected() {
    let env = Env::default();
    let (_admin, client) = setup_test(&env);

    let oracle_pubkey = BytesN::from_array(&env, &[1; 32]);
    let uri = Bytes::from_slice(&env, b"ipfs://oracle1");
    env.mock_all_auths();
    client.register_oracle(&oracle_pubkey, &uri, &OracleType::EdnaLab);

    let polygon_id = BytesN::from_array(&env, &[2; 32]);
    let bbox = BoundingBox {
        min_lat: 0,
        max_lat: 10,
        min_lon: 0,
        max_lon: 10,
    };
    client.register_polygon(
        &polygon_id,
        &uri,
        &bbox,
        &100,
        &1,
        &BytesN::from_array(&env, b"US"),
        &BytesN::from_array(&env, &[3; 32]),
    );

    let signatures = Vec::from_array(
        &env,
        [(oracle_pubkey.clone(), BytesN::from_array(&env, &[0; 64]))],
    );
    let analyses = Vec::new(&env);
    let beneficiary = Address::generate(&env);
    let meth_id = BytesN::from_array(&env, &[0; 8]);

    let data = make_survey_data(
        &env,
        &polygon_id,
        &uri,
        1000,
        &signatures,
        &analyses,
        20,
        50,
        100,
        1,
        2024,
        1,
        &meth_id,
        &beneficiary,
    );

    client.submit_survey(&data);
    // Duplicate - same inputs produce same hash
    client.submit_survey(&data);
}
