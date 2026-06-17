#![allow(dead_code)]
use soroban_sdk::{symbol_short, xdr::ToXdr, Address, Bytes, Env, Symbol};

use crate::types::{Proposal, Stakeholder};

pub fn admin_key() -> Symbol {
    symbol_short!("Admin")
}

pub fn bdc_token_key() -> Symbol {
    symbol_short!("RecT")
}

pub fn mrv_oracle_key() -> Symbol {
    symbol_short!("MrvO")
}

pub fn min_threshold_key() -> Symbol {
    symbol_short!("MinW")
}

pub fn voting_period_key() -> Symbol {
    symbol_short!("VPer")
}

pub fn proposal_counter_key() -> Symbol {
    symbol_short!("PrCN")
}

pub fn stakeholder_counter_key() -> Symbol {
    symbol_short!("StCN")
}

// Admin
pub fn write_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&admin_key(), admin);
}

pub fn read_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&admin_key())
        .expect("approval-gov: Admin not set")
}

// BDC Token
pub fn write_bdc_token(env: &Env, addr: &Address) {
    env.storage().instance().set(&bdc_token_key(), addr);
}

pub fn read_bdc_token(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&bdc_token_key())
        .expect("approval-gov: BDC token not set")
}

pub fn has_bdc_token(env: &Env) -> bool {
    env.storage().instance().has(&bdc_token_key())
}

// MRV Oracle
pub fn write_mrv_oracle(env: &Env, addr: &Address) {
    env.storage().instance().set(&mrv_oracle_key(), addr);
}

pub fn read_mrv_oracle(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&mrv_oracle_key())
        .expect("approval-gov: MRV oracle not set")
}

pub fn has_mrv_oracle(env: &Env) -> bool {
    env.storage().instance().has(&mrv_oracle_key())
}

// Min threshold
pub fn write_min_threshold(env: &Env, min_weight: u32) {
    env.storage()
        .instance()
        .set(&min_threshold_key(), &min_weight);
}

pub fn read_min_threshold(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&min_threshold_key())
        .unwrap_or(0)
}

// Voting period (seconds)
pub fn write_voting_period(env: &Env, secs: u64) {
    env.storage().instance().set(&voting_period_key(), &secs);
}

pub fn read_voting_period(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&voting_period_key())
        .unwrap_or(604800)
}

// Proposal counter
pub fn write_proposal_counter(env: &Env, count: u64) {
    env.storage()
        .instance()
        .set(&proposal_counter_key(), &count);
}

pub fn read_proposal_counter(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&proposal_counter_key())
        .unwrap_or(0)
}

// Stakeholder counter
pub fn write_stakeholder_counter(env: &Env, count: u32) {
    env.storage()
        .instance()
        .set(&stakeholder_counter_key(), &count);
}

pub fn read_stakeholder_counter(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&stakeholder_counter_key())
        .unwrap_or(0)
}

// Stakeholder storage keys (prefix 0x10 + address XDR)
pub fn stakeholder_key(env: &Env, addr: &Address) -> Bytes {
    let mut key = Bytes::new(env);
    key.append(&Bytes::from_slice(env, &[0x10]));
    key.append(&addr.to_xdr(env));
    key
}

pub fn write_stakeholder(env: &Env, addr: &Address, stakeholder: &Stakeholder) {
    let key = stakeholder_key(env, addr);
    env.storage().persistent().set(&key, stakeholder);
}

pub fn read_stakeholder(env: &Env, addr: &Address) -> Option<Stakeholder> {
    let key = stakeholder_key(env, addr);
    env.storage().persistent().get(&key)
}

pub fn has_stakeholder(env: &Env, addr: &Address) -> bool {
    let key = stakeholder_key(env, addr);
    env.storage().persistent().has(&key)
}

// Proposal storage keys (prefix 0x20 + proposal_id big-endian)
pub fn proposal_key(env: &Env, proposal_id: u64) -> Bytes {
    let mut key = Bytes::new(env);
    key.append(&Bytes::from_slice(env, &[0x20]));
    let id_bytes = proposal_id.to_be_bytes();
    key.append(&Bytes::from_slice(env, &id_bytes));
    key
}

pub fn write_proposal(env: &Env, proposal_id: u64, proposal: &Proposal) {
    let key = proposal_key(env, proposal_id);
    env.storage().persistent().set(&key, proposal);
}

pub fn read_proposal(env: &Env, proposal_id: u64) -> Option<Proposal> {
    let key = proposal_key(env, proposal_id);
    env.storage().persistent().get(&key)
}

pub fn has_proposal(env: &Env, proposal_id: u64) -> bool {
    let key = proposal_key(env, proposal_id);
    env.storage().persistent().has(&key)
}
