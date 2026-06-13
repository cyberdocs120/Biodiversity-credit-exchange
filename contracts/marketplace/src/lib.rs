#![no_std]
use soroban_sdk::{contract, contractimpl, panic_with_error, symbol_short, Address, Env, Vec};

mod types;
mod errors;
mod storage;
mod order_book;

#[cfg(test)]
mod test;

pub use crate::types::{Order, OrderSide, OrderRestriction, OrderStatus};
pub use crate::errors::MarketError;
use crate::storage::*;

#[contract]
pub struct MarketplaceContract;

#[contractimpl]
impl MarketplaceContract {
    pub fn __constructor(env: Env, admin: Address) {
        admin.require_auth();
        write_admin(&env, &admin);
        write_fee_rate(&env, 25);
        write_order_counter(&env, 0);
    }

    pub fn admin(env: Env) -> Address {
        read_admin(&env)
    }

    pub fn set_fee_rate(env: Env, rate: u32) {
        read_admin(&env).require_auth();
        if rate > 100 {
            panic_with_error!(&env, MarketError::FeeCapExceeded);
        }
        write_fee_rate(&env, rate);
    }

    pub fn fee_rate(env: Env) -> u32 {
        read_fee_rate(&env)
    }

    pub fn set_bdc_token(env: Env, addr: Address) {
        read_admin(&env).require_auth();
        write_bdc_token(&env, &addr);
    }

    pub fn bdc_token(env: Env) -> Address {
        read_bdc_token(&env)
    }

    pub fn set_usdc_token(env: Env, addr: Address) {
        read_admin(&env).require_auth();
        write_usdc_token(&env, &addr);
    }

    pub fn usdc_token(env: Env) -> Address {
        read_usdc_token(&env)
    }

    pub fn set_fee_vault(env: Env, addr: Address) {
        read_admin(&env).require_auth();
        write_fee_vault(&env, &addr);
    }

    pub fn fee_vault(env: Env) -> Address {
        read_fee_vault(&env)
    }

    pub fn place_order(
        env: Env,
        trader: Address,
        side: OrderSide,
        price: i128,
        qty: u64,
        restrictions: OrderRestriction,
        biome_filter: Option<u32>,
        vintage_filter: Option<u32>,
    ) -> u64 {
        trader.require_auth();

        if qty == 0 {
            panic_with_error!(&env, MarketError::InvalidQuantity);
        }

        let order_id = read_order_counter(&env) + 1;
        write_order_counter(&env, order_id);

        let order = Order {
            order_id,
            trader: trader.clone(),
            side,
            price,
            initial_qty: qty,
            remaining_qty: qty,
            timestamp: env.ledger().timestamp(),
            restrictions,
            biome_filter,
            vintage_filter,
            status: OrderStatus::Open,
        };

        write_order(&env, order_id, &order);

        env.events().publish(
            (symbol_short!("mkt"), symbol_short!("plac")),
            (order_id, trader, side, price, qty),
        );

        order_id
    }

    pub fn cancel_order(env: Env, order_id: u64) {
        let mut order = read_order(&env, order_id).unwrap_or_else(|| {
            panic_with_error!(&env, MarketError::OrderNotFound);
        });

        order.trader.require_auth();

        if order.status != OrderStatus::Open {
            panic_with_error!(&env, MarketError::OrderFilled);
        }

        order.status = OrderStatus::Cancelled;
        write_order(&env, order_id, &order);

        env.events().publish(
            (symbol_short!("mkt"), symbol_short!("canc")),
            order_id,
        );
    }

    pub fn get_order(env: Env, order_id: u64) -> Order {
        read_order(&env, order_id).unwrap_or_else(|| {
            panic_with_error!(&env, MarketError::OrderNotFound);
        })
    }

    pub fn get_best_bid(env: Env) -> Option<Order> {
        order_book::best_bid(&env)
    }

    pub fn get_best_ask(env: Env) -> Option<Order> {
        order_book::best_ask(&env)
    }

    pub fn get_best_bid_for_biome(env: Env, biome: u32) -> Option<Order> {
        order_book::best_bid_for_biome(&env, biome)
    }

    pub fn get_best_ask_for_biome(env: Env, biome: u32) -> Option<Order> {
        order_book::best_ask_for_biome(&env, biome)
    }

    pub fn get_buy_orders(env: Env) -> Vec<Order> {
        order_book::buy_orders(&env)
    }

    pub fn get_sell_orders(env: Env) -> Vec<Order> {
        order_book::sell_orders(&env)
    }
}
