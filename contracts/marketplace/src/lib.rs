#![no_std]
#![allow(clippy::too_many_arguments)]
use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, Address, Env, IntoVal, Vec,
};

mod errors;
mod order_book;
mod storage;
mod types;

#[cfg(test)]
mod test;

pub use crate::errors::MarketError;
use crate::storage::*;
pub use crate::types::{Order, OrderRestriction, OrderSide, OrderStatus};

#[contract]
pub struct MarketplaceContract;

#[contractimpl]
impl MarketplaceContract {
    pub fn initialize(env: Env, admin: Address) {
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

        env.events()
            .publish((symbol_short!("mkt"), symbol_short!("canc")), order_id);
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

    pub fn match_orders(env: Env, buy_id: u64, sell_id: u64) -> (u64, i128, i128) {
        let mut buy_order = read_order(&env, buy_id).unwrap_or_else(|| {
            panic_with_error!(&env, MarketError::OrderNotFound);
        });
        let mut sell_order = read_order(&env, sell_id).unwrap_or_else(|| {
            panic_with_error!(&env, MarketError::OrderNotFound);
        });

        buy_order.trader.require_auth();
        sell_order.trader.require_auth();

        if buy_order.status != OrderStatus::Open || sell_order.status != OrderStatus::Open {
            panic_with_error!(&env, MarketError::OrderFilled);
        }
        if buy_order.side != OrderSide::Buy || sell_order.side != OrderSide::Sell {
            panic_with_error!(&env, MarketError::OrderNotFound);
        }

        if buy_order.price < sell_order.price {
            panic_with_error!(&env, MarketError::PriceMismatch);
        }

        // Biome filter check
        if let (Some(b_buy), Some(b_sell)) = (buy_order.biome_filter, sell_order.biome_filter) {
            if b_buy != b_sell {
                panic_with_error!(&env, MarketError::BiomeMismatch);
            }
        }
        // Vintage filter check
        if let (Some(v_buy), Some(v_sell)) = (buy_order.vintage_filter, sell_order.vintage_filter) {
            if v_buy != v_sell {
                panic_with_error!(&env, MarketError::VintageMismatch);
            }
        }

        // FOK (Fill-or-Kill) enforcement
        if buy_order.restrictions == OrderRestriction::FillOrKill
            && buy_order.remaining_qty > sell_order.remaining_qty
        {
            buy_order.status = OrderStatus::Cancelled;
            write_order(&env, buy_id, &buy_order);
            env.events()
                .publish((symbol_short!("mkt"), symbol_short!("canc")), buy_id);
            return (0, 0, 0);
        }
        if sell_order.restrictions == OrderRestriction::FillOrKill
            && sell_order.remaining_qty > buy_order.remaining_qty
        {
            sell_order.status = OrderStatus::Cancelled;
            write_order(&env, sell_id, &sell_order);
            env.events()
                .publish((symbol_short!("mkt"), symbol_short!("canc")), sell_id);
            return (0, 0, 0);
        }

        let fill_qty = buy_order.remaining_qty.min(sell_order.remaining_qty);
        let fill_price = sell_order.price;
        let fill_value = fill_price * i128::from(fill_qty);

        let fee_rate = read_fee_rate(&env);
        let fee = fill_value * i128::from(fee_rate) / 10000;

        let bdc_id = read_bdc_token(&env);
        let usdc_id = read_usdc_token(&env);
        let fee_vault = read_fee_vault(&env);

        // Find and transfer BDC tokens
        let mut tokens_to_transfer: Vec<u64> = Vec::new(&env);
        let seller_tokens: Vec<u64> = env.invoke_contract(
            &bdc_id,
            &soroban_sdk::Symbol::new(&env, "tokens_by_owner"),
            (sell_order.trader.clone(), 0u64, 100u64).into_val(&env),
        );

        for i in 0..seller_tokens.len() {
            if u64::from(tokens_to_transfer.len()) >= fill_qty {
                break;
            }
            let token_id = seller_tokens.get(i).unwrap();
            let metadata: crate::types::BdcMetadata = env.invoke_contract(
                &bdc_id,
                &soroban_sdk::Symbol::new(&env, "token_metadata"),
                (token_id,).into_val(&env),
            );

            let mut matches = true;
            if let Some(b) = buy_order.biome_filter {
                if metadata.biome as u32 != b {
                    matches = false;
                }
            }
            if let Some(v) = buy_order.vintage_filter {
                if metadata.vintage_year != v {
                    matches = false;
                }
            }
            if let Some(b) = sell_order.biome_filter {
                if metadata.biome as u32 != b {
                    matches = false;
                }
            }
            if let Some(v) = sell_order.vintage_filter {
                if metadata.vintage_year != v {
                    matches = false;
                }
            }

            if matches {
                tokens_to_transfer.push_back(token_id);
            }
        }

        if u64::from(tokens_to_transfer.len()) < fill_qty {
            panic_with_error!(&env, MarketError::InsufficientBalance);
        }

        for i in 0..tokens_to_transfer.len() {
            let token_id = tokens_to_transfer.get(i).unwrap();
            env.invoke_contract::<()>(
                &bdc_id,
                &soroban_sdk::Symbol::new(&env, "transfer"),
                (
                    sell_order.trader.clone(),
                    buy_order.trader.clone(),
                    token_id,
                )
                    .into_val(&env),
            );
        }

        // Transfer USDC
        env.invoke_contract::<()>(
            &usdc_id,
            &soroban_sdk::Symbol::new(&env, "transfer"),
            (
                buy_order.trader.clone(),
                sell_order.trader.clone(),
                fill_value - fee,
            )
                .into_val(&env),
        );
        if fee > 0 {
            env.invoke_contract::<()>(
                &usdc_id,
                &soroban_sdk::Symbol::new(&env, "transfer"),
                (buy_order.trader.clone(), fee_vault, fee).into_val(&env),
            );
        }

        // Update orders
        buy_order.remaining_qty -= fill_qty;
        sell_order.remaining_qty -= fill_qty;

        if buy_order.remaining_qty == 0 {
            buy_order.status = OrderStatus::Filled;
        } else if buy_order.restrictions == OrderRestriction::ImmediateOrCancel {
            buy_order.status = OrderStatus::Cancelled;
        }
        if sell_order.remaining_qty == 0 {
            sell_order.status = OrderStatus::Filled;
        } else if sell_order.restrictions == OrderRestriction::ImmediateOrCancel {
            sell_order.status = OrderStatus::Cancelled;
        }

        write_order(&env, buy_id, &buy_order);
        write_order(&env, sell_id, &sell_order);

        env.events().publish(
            (symbol_short!("mkt"), symbol_short!("matc")),
            (buy_id, sell_id, fill_qty, fill_price, fee),
        );

        (fill_qty, fill_price, fee)
    }

    pub fn auto_match(env: Env) -> u32 {
        let mut match_count = 0;
        loop {
            let best_bid = order_book::best_bid(&env);
            let best_ask = order_book::best_ask(&env);

            if let (Some(bid), Some(ask)) = (best_bid, best_ask) {
                if bid.price >= ask.price {
                    // Check biome filters
                    let mut can_match = true;
                    if let (Some(b_bid), Some(b_ask)) = (bid.biome_filter, ask.biome_filter) {
                        if b_bid != b_ask {
                            can_match = false;
                        }
                    }

                    if can_match {
                        Self::match_orders(env.clone(), bid.order_id, ask.order_id);
                        match_count += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        match_count
    }
}
