#![allow(dead_code)]
use soroban_sdk::{Address, Bytes, BytesN, Env, Symbol, symbol_short};

use crate::types::{OracleNode, HabitatPolygon, SurveyRecord};

pub fn admin_key() -> Symbol {
    symbol_short!("Admin")
}

pub fn paused_key() -> Symbol {
    symbol_short!("Pause")
}

pub fn threshold_n_key() -> Symbol {
    symbol_short!("ThN")
}

pub fn threshold_d_key() -> Symbol {
    symbol_short!("ThD")
}

pub fn oracle_count_key() -> Symbol {
    symbol_short!("OrC")
}

pub fn bdc_token_key() -> Symbol {
    symbol_short!("RecT")
}

pub fn approval_gov_key() -> Symbol {
    symbol_short!("Gov")
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

// Pause
pub fn write_paused(env: &Env, paused: bool) {
    env.storage().instance().set(&paused_key(), &paused);
}

pub fn read_paused(env: &Env) -> bool {
    env.storage().instance().get(&paused_key()).unwrap_or(false)
}

// Threshold
pub fn write_threshold(env: &Env, n: u32, d: u32) {
    env.storage().instance().set(&threshold_n_key(), &n);
    env.storage().instance().set(&threshold_d_key(), &d);
}

pub fn read_threshold(env: &Env) -> (u32, u32) {
    let n = env.storage().instance().get(&threshold_n_key()).unwrap_or(1);
    let d = env.storage().instance().get(&threshold_d_key()).unwrap_or(1);
    (n, d)
}

// Oracle Count
pub fn write_oracle_count(env: &Env, count: u32) {
    env.storage().instance().set(&oracle_count_key(), &count);
}

pub fn read_oracle_count(env: &Env) -> u32 {
    env.storage().instance().get(&oracle_count_key()).unwrap_or(0)
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

// Approval Gov Address
pub fn write_approval_gov(env: &Env, addr: &Address) {
    env.storage().instance().set(&approval_gov_key(), addr);
}

pub fn read_approval_gov(env: &Env) -> Address {
    env.storage().instance().get(&approval_gov_key()).unwrap()
}

pub fn has_approval_gov(env: &Env) -> bool {
    env.storage().instance().has(&approval_gov_key())
}

// Oracle storage keys (prefix 0x10 + pubkey)
pub fn oracle_key(env: &Env, pubkey: &BytesN<32>) -> Bytes {
    let mut key = Bytes::new(env);
    key.append(&Bytes::from_slice(env, &[0x10]));
    key.append(&Bytes::from_slice(env, &pubkey.to_array()));
    key
}

pub fn write_oracle(env: &Env, pubkey: &BytesN<32>, node: &OracleNode) {
    let key = oracle_key(env, pubkey);
    env.storage().persistent().set(&key, node);
}

pub fn read_oracle(env: &Env, pubkey: &BytesN<32>) -> Option<OracleNode> {
    let key = oracle_key(env, pubkey);
    env.storage().persistent().get(&key)
}

pub fn has_oracle(env: &Env, pubkey: &BytesN<32>) -> bool {
    let key = oracle_key(env, pubkey);
    env.storage().persistent().has(&key)
}

// Polygon storage keys (prefix 0x20 + polygon_id)
pub fn polygon_key(env: &Env, polygon_id: &BytesN<32>) -> Bytes {
    let mut key = Bytes::new(env);
    key.append(&Bytes::from_slice(env, &[0x20]));
    key.append(&Bytes::from_slice(env, &polygon_id.to_array()));
    key
}

pub fn write_polygon(env: &Env, polygon_id: &BytesN<32>, polygon: &HabitatPolygon) {
    let key = polygon_key(env, polygon_id);
    env.storage().persistent().set(&key, polygon);
}

pub fn read_polygon(env: &Env, polygon_id: &BytesN<32>) -> Option<HabitatPolygon> {
    let key = polygon_key(env, polygon_id);
    env.storage().persistent().get(&key)
}

pub fn has_polygon(env: &Env, polygon_id: &BytesN<32>) -> bool {
    let key = polygon_key(env, polygon_id);
    env.storage().persistent().has(&key)
}

// Survey storage keys (prefix 0x30 + survey_hash)
pub fn survey_key(env: &Env, survey_hash: &BytesN<32>) -> Bytes {
    let mut key = Bytes::new(env);
    key.append(&Bytes::from_slice(env, &[0x30]));
    key.append(&Bytes::from_slice(env, &survey_hash.to_array()));
    key
}

pub fn write_survey(env: &Env, survey_hash: &BytesN<32>, survey: &SurveyRecord) {
    let key = survey_key(env, survey_hash);
    env.storage().persistent().set(&key, survey);
}

pub fn read_survey(env: &Env, survey_hash: &BytesN<32>) -> Option<SurveyRecord> {
    let key = survey_key(env, survey_hash);
    env.storage().persistent().get(&key)
}

pub fn has_survey(env: &Env, survey_hash: &BytesN<32>) -> bool {
    let key = survey_key(env, survey_hash);
    env.storage().persistent().has(&key)
}
