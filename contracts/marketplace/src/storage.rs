use soroban_sdk::{symbol_short, Address, Env, Symbol};
use crate::types::Order;

pub fn admin_key() -> Symbol { symbol_short!("Admin") }
pub fn fee_rate_key() -> Symbol { symbol_short!("FeeRt") }
pub fn order_counter_key() -> Symbol { symbol_short!("OrCN") }
pub fn bdc_token_key() -> Symbol { symbol_short!("RecT") }
pub fn usdc_token_key() -> Symbol { symbol_short!("USDC") }
pub fn fee_vault_key() -> Symbol { symbol_short!("FVal") }

pub fn write_admin(env: &Env, addr: &Address) {
    env.storage().instance().set(&admin_key(), addr);
}

pub fn read_admin(env: &Env) -> Address {
    env.storage().instance().get(&admin_key()).expect("marketplace: Admin not set")
}

pub fn write_fee_rate(env: &Env, rate: u32) {
    env.storage().instance().set(&fee_rate_key(), &rate);
}

pub fn read_fee_rate(env: &Env) -> u32 {
    env.storage().instance().get(&fee_rate_key()).unwrap_or(25)
}

pub fn write_order_counter(env: &Env, count: u64) {
    env.storage().instance().set(&order_counter_key(), &count);
}

pub fn read_order_counter(env: &Env) -> u64 {
    env.storage().instance().get(&order_counter_key()).unwrap_or(0)
}

pub fn write_bdc_token(env: &Env, addr: &Address) {
    env.storage().instance().set(&bdc_token_key(), addr);
}

pub fn read_bdc_token(env: &Env) -> Address {
    env.storage().instance().get(&bdc_token_key()).expect("marketplace: BDC token not set")
}

pub fn write_usdc_token(env: &Env, addr: &Address) {
    env.storage().instance().set(&usdc_token_key(), addr);
}

pub fn read_usdc_token(env: &Env) -> Address {
    env.storage().instance().get(&usdc_token_key()).expect("marketplace: USDC token not set")
}

pub fn write_fee_vault(env: &Env, addr: &Address) {
    env.storage().instance().set(&fee_vault_key(), addr);
}

pub fn read_fee_vault(env: &Env) -> Address {
    env.storage().instance().get(&fee_vault_key()).expect("marketplace: Fee vault not set")
}

pub fn order_key(id: u64) -> (Symbol, u64) {
    (symbol_short!("order"), id)
}

pub fn write_order(env: &Env, id: u64, order: &Order) {
    env.storage().persistent().set(&order_key(id), order);
}

pub fn read_order(env: &Env, id: u64) -> Option<Order> {
    env.storage().persistent().get(&order_key(id))
}
