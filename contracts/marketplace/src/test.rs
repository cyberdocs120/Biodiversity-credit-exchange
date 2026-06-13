#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{Address, Env};

fn setup() -> (Env, Address, MarketplaceContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let contract_id = env.register(MarketplaceContract, (&admin,));
    let client = MarketplaceContractClient::new(&env, &contract_id);
    
    (env, admin, client)
}

#[test]
fn test_place_buy_order() {
    let (env, _, client) = setup();
    let trader = Address::generate(&env);
    
    let order_id = client.place_order(
        &trader,
        &OrderSide::Buy,
        &100,
        &10,
        &OrderRestriction::None,
        &None,
        &None,
    );
    
    assert_eq!(order_id, 1);
    let order = client.get_order(&order_id);
    assert_eq!(order.trader, trader);
    assert_eq!(order.side, OrderSide::Buy);
    assert_eq!(order.price, 100);
    assert_eq!(order.initial_qty, 10);
    assert_eq!(order.status, OrderStatus::Open);
}

#[test]
fn test_place_sell_order() {
    let (env, _, client) = setup();
    let trader = Address::generate(&env);
    
    let order_id = client.place_order(
        &trader,
        &OrderSide::Sell,
        &120,
        &5,
        &OrderRestriction::None,
        &None,
        &None,
    );
    
    assert_eq!(order_id, 1);
    let order = client.get_order(&order_id);
    assert_eq!(order.side, OrderSide::Sell);
    assert_eq!(order.price, 120);
}

#[test]
fn test_cancel_order() {
    let (env, _, client) = setup();
    let trader = Address::generate(&env);
    
    let order_id = client.place_order(
        &trader,
        &OrderSide::Buy,
        &100,
        &10,
        &OrderRestriction::None,
        &None,
        &None,
    );
    
    client.cancel_order(&order_id);
    let order = client.get_order(&order_id);
    assert_eq!(order.status, OrderStatus::Cancelled);
}

#[test]
fn test_best_bid_ask() {
    let (env, _, client) = setup();
    let t1 = Address::generate(&env);
    let t2 = Address::generate(&env);
    
    client.place_order(&t1, &OrderSide::Buy, &100, &10, &OrderRestriction::None, &None, &None);
    client.place_order(&t2, &OrderSide::Sell, &110, &10, &OrderRestriction::None, &None, &None);
    
    let best_bid = client.get_best_bid().unwrap();
    let best_ask = client.get_best_ask().unwrap();
    
    assert_eq!(best_bid.price, 100);
    assert_eq!(best_ask.price, 110);
}

#[test]
fn test_price_time_priority() {
    let (env, _, client) = setup();
    let t1 = Address::generate(&env);
    let t2 = Address::generate(&env);
    let t3 = Address::generate(&env);
    
    // t1 buys at 100
    env.ledger().set_timestamp(1000);
    client.place_order(&t1, &OrderSide::Buy, &100, &10, &OrderRestriction::None, &None, &None);
    
    // t2 buys at 110 (better price)
    env.ledger().set_timestamp(2000);
    client.place_order(&t2, &OrderSide::Buy, &110, &10, &OrderRestriction::None, &None, &None);
    
    // t3 buys at 100 (same price as t1, but later)
    env.ledger().set_timestamp(3000);
    client.place_order(&t3, &OrderSide::Buy, &100, &10, &OrderRestriction::None, &None, &None);
    
    let buys = client.get_buy_orders();
    assert_eq!(buys.get(0).unwrap().trader, t2); // Highest price
    assert_eq!(buys.get(1).unwrap().trader, t1); // Same price, earlier
    assert_eq!(buys.get(2).unwrap().trader, t3); // Same price, later
}

#[test]
fn test_biome_filtered_orders() {
    let (env, _, client) = setup();
    let t1 = Address::generate(&env);
    let t2 = Address::generate(&env);
    
    // t1 sells Biome 1
    client.place_order(&t1, &OrderSide::Sell, &100, &10, &OrderRestriction::None, &Some(1), &None);
    // t2 sells Biome 2
    client.place_order(&t2, &OrderSide::Sell, &110, &10, &OrderRestriction::None, &Some(2), &None);
    
    let best_biome_1 = client.get_best_ask_for_biome(&1).unwrap();
    assert_eq!(best_biome_1.trader, t1);
    
    let best_biome_2 = client.get_best_ask_for_biome(&2).unwrap();
    assert_eq!(best_biome_2.trader, t2);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_cancel_nonexistent() {
    let (_env, _, client) = setup();
    client.cancel_order(&99);
}
