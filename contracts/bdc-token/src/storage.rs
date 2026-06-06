use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env, Symbol, symbol_short};

use crate::types::BdcTokenValue;

pub fn admin_key() -> Symbol {
    symbol_short!("Admin")
}

pub fn token_id_counter_key() -> Symbol {
    symbol_short!("TIDC")
}

pub fn total_supply_key() -> Symbol {
    symbol_short!("TSup")
}

pub fn authorized_minter_key() -> Symbol {
    symbol_short!("AMnt")
}

pub fn authorized_burner_key() -> Symbol {
    symbol_short!("ABrn")
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

// Token ID counter
pub fn write_token_id_counter(env: &Env, counter: u64) {
    env.storage().instance().set(&token_id_counter_key(), &counter);
}

pub fn read_token_id_counter(env: &Env) -> u64 {
    env.storage().instance().get(&token_id_counter_key()).unwrap_or(0)
}

// Total supply
pub fn write_total_supply(env: &Env, supply: u64) {
    env.storage().instance().set(&total_supply_key(), &supply);
}

pub fn read_total_supply(env: &Env) -> u64 {
    env.storage().instance().get(&total_supply_key()).unwrap_or(0)
}

// Authorized minter
pub fn write_authorized_minter(env: &Env, minter: &Address) {
    env.storage().instance().set(&authorized_minter_key(), minter);
}

pub fn read_authorized_minter(env: &Env) -> Address {
    env.storage().instance().get(&authorized_minter_key()).unwrap()
}

pub fn has_authorized_minter(env: &Env) -> bool {
    env.storage().instance().has(&authorized_minter_key())
}

// Authorized burner
pub fn write_authorized_burner(env: &Env, burner: &Address) {
    env.storage().instance().set(&authorized_burner_key(), burner);
}

pub fn read_authorized_burner(env: &Env) -> Address {
    env.storage().instance().get(&authorized_burner_key()).unwrap()
}

pub fn has_authorized_burner(env: &Env) -> bool {
    env.storage().instance().has(&authorized_burner_key())
}

// Token storage keys (prefix 0x01 + token_id big-endian)
pub fn token_storage_key(env: &Env, token_id: u64) -> Bytes {
    let mut key = Bytes::new(env);
    key.append(&Bytes::from_slice(env, &[0x01]));
    let id_bytes = token_id.to_be_bytes();
    key.append(&Bytes::from_slice(env, &id_bytes));
    key
}

pub fn write_token(env: &Env, token_id: u64, token: &BdcTokenValue) {
    let key = token_storage_key(env, token_id);
    env.storage().persistent().set(&key, token);
}

pub fn read_token(env: &Env, token_id: u64) -> Option<BdcTokenValue> {
    let key = token_storage_key(env, token_id);
    env.storage().persistent().get(&key)
}

pub fn has_token(env: &Env, token_id: u64) -> bool {
    let key = token_storage_key(env, token_id);
    env.storage().persistent().has(&key)
}

// Owner count keys (prefix 0x02 + owner address XDR)
pub fn owner_count_key(env: &Env, owner: &Address) -> Bytes {
    let mut key = Bytes::new(env);
    key.append(&Bytes::from_slice(env, &[0x02]));
    key.append(&owner.to_xdr(env));
    key
}

pub fn write_owner_count(env: &Env, owner: &Address, count: u64) {
    let key = owner_count_key(env, owner);
    env.storage().persistent().set(&key, &count);
}

pub fn read_owner_count(env: &Env, owner: &Address) -> u64 {
    let key = owner_count_key(env, owner);
    env.storage().persistent().get(&key).unwrap_or(0)
}

// Polygon token count keys (prefix 0x03 + polygon_id XDR)
pub fn polygon_token_count_key(env: &Env, polygon_id: &BytesN<32>) -> Bytes {
    let mut key = Bytes::new(env);
    key.append(&Bytes::from_slice(env, &[0x03]));
    key.append(&polygon_id.to_xdr(env));
    key
}

pub fn write_polygon_token_count(env: &Env, polygon_id: &BytesN<32>, count: u64) {
    let key = polygon_token_count_key(env, polygon_id);
    env.storage().persistent().set(&key, &count);
}

pub fn read_polygon_token_count(env: &Env, polygon_id: &BytesN<32>) -> u64 {
    let key = polygon_token_count_key(env, polygon_id);
    env.storage().persistent().get(&key).unwrap_or(0)
}
