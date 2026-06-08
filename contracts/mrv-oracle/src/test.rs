#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Bytes, BytesN, Env};

fn setup_test(env: &Env) -> (Address, MrvOracleContractClient<'static>) {
    let admin = Address::generate(env);
    let contract_id = env.register(MrvOracleContract, (&admin,));
    let client = MrvOracleContractClient::new(env, &contract_id);
    (admin, client)
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
    client.register_polygon(&polygon_id, &geometry_cid, &bbox, &area_ha, &biome, &country, &project_id);

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
    client.register_polygon(&polygon_id, &geometry_cid, &bbox, &1000, &1, &BytesN::from_array(&env, b"BR"), &BytesN::from_array(&env, &[3; 32]));
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
