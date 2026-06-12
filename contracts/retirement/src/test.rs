#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, testutils::Ledger as _, Address, Bytes, BytesN, Env, Vec};

use bdc_token::{BdcTokenContract, BdcTokenContractClient};
use bdc_token::types::{Biome, MintParams};

fn setup() -> (Env, Address, RetirementContractClient<'static>, BdcTokenContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let retirer = Address::generate(&env);

    let bdc_id = env.register(BdcTokenContract, (&admin,));
    let bdc_client = BdcTokenContractClient::new(&env, &bdc_id);

    let retire_id = env.register(RetirementContract, (&admin,));
    let retire_client = RetirementContractClient::new(&env, &retire_id);

    bdc_client.authorize_burner(&retire_id);
    retire_client.set_bdc_token(&bdc_id);

    (env, admin, retire_client, bdc_client, retirer)
}

fn mint_tokens(env: &Env, client: &BdcTokenContractClient, to: &Address, count: u64) -> Vec<u64> {
    let params = MintParams {
        polygon_id: BytesN::from_array(env, &[1u8; 32]),
        methodology_id: BytesN::from_array(env, &[2u8; 8]),
        survey_ipfs_cid: Bytes::from_slice(env, b"QmTest"),
        baseline_bsi: 28,
        current_bsi: 64,
        area_ha_contribution: 100,
        biome: Biome::TropicalForest,
        vintage_year: 2025,
        vintage_quarter: 2,
        approval_governance_id: BytesN::from_array(env, &[3u8; 32]),
    };

    let mut ids: Vec<u64> = Vec::new(env);
    for _ in 0..count {
        let id = client.mint(to, &params);
        ids.push_back(id);
    }
    ids
}

fn sample_claim_data(env: &Env) -> ClaimData {
    ClaimData {
        period_start: 2024001,
        period_end: 2024365,
        purpose: Bytes::from_slice(env, b"conservation"),
        jurisdiction: Bytes::from_slice(env, b"BR"),
    }
}

#[test]
fn test_retire_tokens() {
    let (env, _admin, retire_client, bdc_client, retirer) = setup();
    env.ledger().set_timestamp(5000);
    env.ledger().set_sequence_number(100);
    let token_ids = mint_tokens(&env, &bdc_client, &retirer, 3);
    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let claim = sample_claim_data(&env);

    let receipt_id = retire_client.retire(&retirer, &token_ids, &polygon_id, &claim);

    let receipt = retire_client.get_receipt(&receipt_id);
    assert_eq!(receipt.retirer, retirer);
    assert_eq!(receipt.total_credits, 3);
    assert_eq!(receipt.polygon_id, polygon_id);
    assert_eq!(receipt.claim_period_start, 2024001);
    assert_eq!(receipt.claim_period_end, 2024365);
    assert_eq!(receipt.token_ids.len(), 3);
    assert!(receipt.timestamp > 0);
    assert!(receipt.block_height > 0);

    assert!(retire_client.is_token_retired(&1));
    assert!(retire_client.is_token_retired(&2));
    assert!(retire_client.is_token_retired(&3));

    assert_eq!(bdc_client.total_supply(), 0);
}

#[test]
fn test_retire_with_polygon_binding() {
    let (env, _admin, retire_client, bdc_client, retirer) = setup();
    env.ledger().set_timestamp(5000);
    let token_ids = mint_tokens(&env, &bdc_client, &retirer, 2);
    let polygon_id = BytesN::from_array(&env, &[5u8; 32]);
    let claim = sample_claim_data(&env);

    let receipt_id = retire_client.retire(&retirer, &token_ids, &polygon_id, &claim);
    let receipt = retire_client.get_receipt(&receipt_id);

    assert_eq!(receipt.polygon_id, polygon_id);
    assert_eq!(receipt.total_credits, 2);
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn test_empty_token_list_rejected() {
    let (env, _admin, retire_client, _bdc_client, retirer) = setup();
    env.ledger().set_timestamp(5000);
    let empty: Vec<u64> = Vec::new(&env);
    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let claim = sample_claim_data(&env);

    retire_client.retire(&retirer, &empty, &polygon_id, &claim);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_double_retire_rejected() {
    let (env, _admin, retire_client, bdc_client, retirer) = setup();
    env.ledger().set_timestamp(5000);
    let token_ids = mint_tokens(&env, &bdc_client, &retirer, 1);
    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let claim = sample_claim_data(&env);

    retire_client.retire(&retirer, &token_ids, &polygon_id, &claim);
    retire_client.retire(&retirer, &token_ids, &polygon_id, &claim);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_get_nonexistent_receipt() {
    let (env, _admin, retire_client, _bdc_client, _retirer) = setup();
    let fake_id = BytesN::from_array(&env, &[0u8; 32]);
    retire_client.get_receipt(&fake_id);
}

#[test]
fn test_verify_retirement() {
    let (env, _admin, retire_client, bdc_client, retirer) = setup();
    let token_ids = mint_tokens(&env, &bdc_client, &retirer, 1);
    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let claim = sample_claim_data(&env);

    env.ledger().set_timestamp(5000);
    assert!(!retire_client.verify_retirement(&1));
    retire_client.retire(&retirer, &token_ids, &polygon_id, &claim);
    assert!(retire_client.verify_retirement(&1));
}

#[test]
fn test_verify_claim() {
    let (env, _admin, retire_client, bdc_client, retirer) = setup();
    let token_ids = mint_tokens(&env, &bdc_client, &retirer, 2);
    let polygon_id = BytesN::from_array(&env, &[1u8; 32]);
    let claim = sample_claim_data(&env);

    env.ledger().set_timestamp(5000);
    retire_client.retire(&retirer, &token_ids, &polygon_id, &claim);

    let result = retire_client.verify_claim(&polygon_id, &2024001, &2024365, &retirer);
    assert!(result);

    let wrong_result = retire_client.verify_claim(&polygon_id, &2024002, &2024365, &retirer);
    assert!(!wrong_result);
}

#[test]
fn test_admin_management() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let new_admin = Address::generate(&env);

    let contract_id = env.register(RetirementContract, (&admin,));
    let client = RetirementContractClient::new(&env, &contract_id);

    assert_eq!(client.admin(), admin);

    client.transfer_admin(&new_admin);
    assert_eq!(client.admin(), new_admin);
}
