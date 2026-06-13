#![no_std]
use soroban_sdk::{contract, contractimpl, panic_with_error, symbol_short, xdr::ToXdr, Address, Bytes, BytesN, Env, IntoVal, Vec};

mod errors;
mod geometry;
mod merkle;
mod storage;
mod types;

#[cfg(test)]
mod test;

pub use crate::errors::RetirementError;
pub use crate::types::{ClaimData, Point, RetirementReceipt};
use crate::storage::*;

#[contract]
pub struct RetirementContract;

#[contractimpl]
impl RetirementContract {
    pub fn __constructor(env: Env, admin: Address) {
        admin.require_auth();
        write_admin(&env, &admin);
        write_receipt_counter(&env, 0);
    }

    pub fn admin(env: Env) -> Address {
        read_admin(&env)
    }

    pub fn transfer_admin(env: Env, new_admin: Address) {
        let current_admin = read_admin(&env);
        current_admin.require_auth();
        new_admin.require_auth();
        write_admin(&env, &new_admin);
    }

    pub fn set_bdc_token(env: Env, addr: Address) {
        read_admin(&env).require_auth();
        write_bdc_token(&env, &addr);
    }

    pub fn bdc_token(env: Env) -> Address {
        read_bdc_token(&env)
    }

    pub fn retire(
        env: Env,
        retirer: Address,
        token_ids: Vec<u64>,
        polygon_id: BytesN<32>,
        claim_data: ClaimData,
    ) -> BytesN<32> {
        retirer.require_auth();

        if token_ids.len() == 0 {
            panic_with_error!(&env, RetirementError::EmptyTokenList);
        }

        for i in 0..token_ids.len() {
            let token_id = token_ids.get(i).unwrap();
            if read_retired(&env, token_id) {
                panic_with_error!(&env, RetirementError::TokenAlreadyRetired);
            }
        }

        let bdc_id = read_bdc_token(&env);
        let timestamp = env.ledger().timestamp();
        let block_height = env.ledger().sequence();

        for i in 0..token_ids.len() {
            let token_id = token_ids.get(i).unwrap();
            let _: () = env.invoke_contract(
                &bdc_id,
                &symbol_short!("burn"),
                (retirer.clone(), token_id).into_val(&env),
            );
            write_retired(&env, token_id);
        }

        let total_credits = token_ids.len() as u64;

        let merkle_root = merkle::compute_root(&env, &token_ids);
        let receipt_id = compute_receipt_id(&env, &polygon_id, &retirer, timestamp, &token_ids);

        let receipt = RetirementReceipt {
            receipt_id: receipt_id.clone(),
            retirer: retirer.clone(),
            token_ids: token_ids.clone(),
            polygon_id: polygon_id.clone(),
            total_credits,
            claim_period_start: claim_data.period_start,
            claim_period_end: claim_data.period_end,
            purpose: claim_data.purpose,
            jurisdiction: claim_data.jurisdiction,
            merkle_root,
            timestamp,
            block_height: block_height as u64,
        };

        write_receipt(&env, &receipt_id, &receipt);

        let mut existing = read_claim_index(&env, &polygon_id, &retirer);
        existing.push_back(receipt_id.clone());
        write_claim_index(&env, &polygon_id, &retirer, &existing);

        let counter = read_receipt_counter(&env) + 1;
        write_receipt_counter(&env, counter);

        env.events().publish(
            (symbol_short!("retr"), symbol_short!("done")),
            (receipt_id.clone(), polygon_id, token_ids.len() as u64, total_credits),
        );

        receipt_id
    }

    pub fn get_receipt(env: Env, receipt_id: BytesN<32>) -> RetirementReceipt {
        read_receipt(&env, &receipt_id).unwrap_or_else(|| {
            panic_with_error!(&env, RetirementError::ReceiptNotFound);
        })
    }

    pub fn verify_retirement(env: Env, token_id: u64) -> bool {
        read_retired(&env, token_id)
    }

    pub fn is_token_retired(env: Env, token_id: u64) -> bool {
        read_retired(&env, token_id)
    }

    pub fn verify_claim(
        env: Env,
        polygon_id: BytesN<32>,
        claim_period_start: u64,
        claim_period_end: u64,
        retirer: Address,
    ) -> bool {
        let receipt_ids = read_claim_index(&env, &polygon_id, &retirer);
        for i in 0..receipt_ids.len() {
            let rid = receipt_ids.get(i).unwrap();
            if let Some(receipt) = read_receipt(&env, &rid) {
                if receipt.claim_period_start == claim_period_start
                    && receipt.claim_period_end == claim_period_end
                {
                    return true;
                }
            }
        }
        false
    }

    pub fn prove_claim(
        env: Env,
        retirer: Address,
        polygon_id: BytesN<32>,
        period_start: u64,
        period_end: u64,
        token_index: u32,
    ) -> (BytesN<32>, Vec<BytesN<32>>, u32) {
        let receipt_ids = read_claim_index(&env, &polygon_id, &retirer);
        for i in 0..receipt_ids.len() {
            let rid = receipt_ids.get(i).unwrap();
            if let Some(receipt) = read_receipt(&env, &rid) {
                if receipt.claim_period_start == period_start
                    && receipt.claim_period_end == period_end
                {
                    let proof = merkle::generate_proof(&env, &receipt.token_ids, token_index);
                    return (receipt.merkle_root, proof, token_index);
                }
            }
        }
        panic_with_error!(&env, RetirementError::ReceiptNotFound);
    }

    pub fn prove_polygon_containment(
        _env: Env,
        point: Point,
        polygon: Vec<Point>,
    ) -> bool {
        geometry::point_in_polygon(point, polygon)
    }
}

fn compute_receipt_id(
    env: &Env,
    polygon_id: &BytesN<32>,
    retirer: &Address,
    timestamp: u64,
    token_ids: &Vec<u64>,
) -> BytesN<32> {
    let mut hash_input = Bytes::new(env);
    hash_input.append(&Bytes::from_slice(env, &polygon_id.to_array()));
    hash_input.append(&retirer.to_xdr(env));
    let ts_bytes = timestamp.to_be_bytes();
    hash_input.append(&Bytes::from_slice(env, &ts_bytes));

    let mut tid_hash_input = Bytes::new(env);
    for i in 0..token_ids.len() {
        let id = token_ids.get(i).unwrap();
        let id_bytes = id.to_be_bytes();
        tid_hash_input.append(&Bytes::from_slice(env, &id_bytes));
    }
    let tid_hash: BytesN<32> = env.crypto().sha256(&tid_hash_input).into();
    hash_input.append(&Bytes::from_slice(env, &tid_hash.to_array()));

    env.crypto().sha256(&hash_input).into()
}

