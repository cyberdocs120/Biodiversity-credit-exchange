#![allow(dead_code)]
use soroban_sdk::{symbol_short, xdr::ToXdr, Address, Bytes, BytesN, Env, Symbol, Vec};

use crate::types::RetirementReceipt;

pub fn admin_key() -> Symbol {
    symbol_short!("Admin")
}

pub fn bdc_token_key() -> Symbol {
    symbol_short!("RecT")
}

pub fn receipt_counter_key() -> Symbol {
    symbol_short!("RcCN")
}

// Admin
pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&admin_key(), admin);
}

pub fn read_admin(env: &Env) -> Address {
    env.storage().instance().get(&admin_key()).unwrap()
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&admin_key())
}

// BDC Token Address
pub fn write_bdc_token(env: &Env, addr: &Address) {
    env.storage().instance().set(&bdc_token_key(), addr);
}

pub fn read_bdc_token(env: &Env) -> Address {
    env.storage().instance().get(&bdc_token_key()).unwrap()
}

pub fn has_bdc_token(env: &Env) -> bool {
    env.storage().instance().has(&bdc_token_key())
}

// Receipt counter
pub fn write_receipt_counter(env: &Env, counter: u64) {
    env.storage()
        .instance()
        .set(&receipt_counter_key(), &counter);
}

pub fn read_receipt_counter(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&receipt_counter_key())
        .unwrap_or(0)
}

// Receipt storage keys (prefix 0x10 + receipt_id)
pub fn receipt_key(env: &Env, receipt_id: &BytesN<32>) -> Bytes {
    let mut key = Bytes::new(env);
    key.append(&Bytes::from_slice(env, &[0x10]));
    key.append(&Bytes::from_slice(env, &receipt_id.to_array()));
    key
}

pub fn write_receipt(env: &Env, receipt_id: &BytesN<32>, receipt: &RetirementReceipt) {
    let key = receipt_key(env, receipt_id);
    env.storage().persistent().set(&key, receipt);
}

pub fn read_receipt(env: &Env, receipt_id: &BytesN<32>) -> Option<RetirementReceipt> {
    let key = receipt_key(env, receipt_id);
    env.storage().persistent().get(&key)
}

pub fn has_receipt(env: &Env, receipt_id: &BytesN<32>) -> bool {
    let key = receipt_key(env, receipt_id);
    env.storage().persistent().has(&key)
}

// Token retired flag (prefix 0x20 + token_id big-endian)
pub fn retired_key(env: &Env, token_id: u64) -> Bytes {
    let mut key = Bytes::new(env);
    key.append(&Bytes::from_slice(env, &[0x20]));
    let id_bytes = token_id.to_be_bytes();
    key.append(&Bytes::from_slice(env, &id_bytes));
    key
}

pub fn write_retired(env: &Env, token_id: u64) {
    let key = retired_key(env, token_id);
    env.storage().persistent().set(&key, &true);
}

pub fn read_retired(env: &Env, token_id: u64) -> bool {
    let key = retired_key(env, token_id);
    env.storage().persistent().get(&key).unwrap_or(false)
}

// Claim index: (polygon_id, retirer) -> Vec<receipt_id>
// Prefix 0x30 + polygon_id + retirer XDR
pub fn claim_index_key(env: &Env, polygon_id: &BytesN<32>, retirer: &Address) -> Bytes {
    let mut key = Bytes::new(env);
    key.append(&Bytes::from_slice(env, &[0x30]));
    key.append(&Bytes::from_slice(env, &polygon_id.to_array()));
    key.append(&retirer.to_xdr(env));
    key
}

pub fn write_claim_index(
    env: &Env,
    polygon_id: &BytesN<32>,
    retirer: &Address,
    receipt_ids: &Vec<BytesN<32>>,
) {
    let key = claim_index_key(env, polygon_id, retirer);
    env.storage().persistent().set(&key, receipt_ids);
}

pub fn read_claim_index(env: &Env, polygon_id: &BytesN<32>, retirer: &Address) -> Vec<BytesN<32>> {
    let key = claim_index_key(env, polygon_id, retirer);
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env))
}
