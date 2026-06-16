#![cfg(test)]
use super::*;
use soroban_sdk::testutils::{Address as _, Ledger};
use soroban_sdk::{Address, Env, contract, contractimpl};
use bdc_token::{BdcTokenContract, BdcTokenContractClient, types::MintParams};

fn setup() -> (Env, Address, MarketplaceContractClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let contract_id = env.register(MarketplaceContract, ());
    let client = MarketplaceContractClient::new(&env, &contract_id);
    client.initialize(&admin);
    
    (env, admin, client)
}

#[contract]
struct MockToken;
#[contractimpl]
impl MockToken {
    pub fn transfer(_env: Env, _from: Address, _to: Address, _amount: i128) {}
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

fn setup_full<'a>(env: &'a Env, client: &MarketplaceContractClient<'a>) -> (Address, BdcTokenContractClient<'a>, Address) {
    let admin = client.admin();
    let bdc_id = env.register(BdcTokenContract, ());
    let bdc_client = BdcTokenContractClient::new(env, &bdc_id);
    bdc_client.initialize(&admin);
    let usdc_id = env.register(MockToken, ());
    let fee_vault = Address::generate(env);
    client.set_bdc_token(&bdc_id);
    client.set_usdc_token(&usdc_id);
    client.set_fee_vault(&fee_vault);
    bdc_client.authorize_minter(&admin);
    (bdc_id, bdc_client, fee_vault)
}

fn mint_token(env: &Env, bdc_client: &BdcTokenContractClient, to: &Address, biome: u8) {
    let polygon_id = soroban_sdk::BytesN::from_array(env, &[0u8; 32]);
    let biome_enum = match biome {
        0 => bdc_token::types::Biome::TropicalForest,
        1 => bdc_token::types::Biome::TemperateForest,
        2 => bdc_token::types::Biome::Grassland,
        _ => bdc_token::types::Biome::Other,
    };
    bdc_client.mint(to, &MintParams {
        polygon_id,
        methodology_id: soroban_sdk::BytesN::from_array(env, &[0u8; 8]),
        survey_ipfs_cid: soroban_sdk::Bytes::new(env),
        baseline_bsi: 50,
        current_bsi: 80,
        area_ha_contribution: 100,
        biome: biome_enum,
        vintage_year: 2024,
        vintage_quarter: 1,
        approval_governance_id: Address::generate(env),
    });
}

#[test]
fn test_match_orders_basic() {
    let (env, _admin, client) = setup();
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    let (_bdc_id, bdc_client, _fv) = setup_full(&env, &client);
    
    mint_token(&env, &bdc_client, &seller, 0);
    
    let sell_id = client.place_order(&seller, &OrderSide::Sell, &100, &1, &OrderRestriction::None, &None, &None);
    let buy_id = client.place_order(&buyer, &OrderSide::Buy, &100, &1, &OrderRestriction::None, &None, &None);
    
    client.match_orders(&buy_id, &sell_id);
    
    let sell_order = client.get_order(&sell_id);
    let buy_order = client.get_order(&buy_id);
    assert_eq!(sell_order.status, OrderStatus::Filled);
    assert_eq!(buy_order.status, OrderStatus::Filled);
    
    assert_eq!(bdc_client.owner_of(&1u64), buyer);
}

#[test]
fn test_match_partial_fill() {
    let (env, _admin, client) = setup();
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    let (_bdc_id, bdc_client, _fv) = setup_full(&env, &client);
    
    for _ in 0..10 {
        mint_token(&env, &bdc_client, &seller, 0);
    }
    
    let sell_id = client.place_order(&seller, &OrderSide::Sell, &100, &5, &OrderRestriction::None, &None, &None);
    let buy_id = client.place_order(&buyer, &OrderSide::Buy, &100, &10, &OrderRestriction::None, &None, &None);
    
    client.match_orders(&buy_id, &sell_id);
    
    let sell_order = client.get_order(&sell_id);
    let buy_order = client.get_order(&buy_id);
    
    assert_eq!(sell_order.status, OrderStatus::Filled);
    assert_eq!(sell_order.remaining_qty, 0);
    assert_eq!(buy_order.status, OrderStatus::Open);
    assert_eq!(buy_order.remaining_qty, 5);
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_price_mismatch_rejected() {
    let (env, _, client) = setup();
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    
    client.place_order(&seller, &OrderSide::Sell, &100, &1, &OrderRestriction::None, &None, &None);
    client.place_order(&buyer, &OrderSide::Buy, &90, &1, &OrderRestriction::None, &None, &None);
    
    client.match_orders(&2, &1);
}

#[test]
fn test_biome_filter_match() {
    let (env, _admin, client) = setup();
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    let (_bdc_id, bdc_client, _fv) = setup_full(&env, &client);
    
    mint_token(&env, &bdc_client, &seller, 0);
    
    let sell_id = client.place_order(&seller, &OrderSide::Sell, &100, &1, &OrderRestriction::None, &Some(0), &None);
    let buy_id = client.place_order(&buyer, &OrderSide::Buy, &100, &1, &OrderRestriction::None, &Some(0), &None);
    
    client.match_orders(&buy_id, &sell_id);
    
    let sell_order = client.get_order(&sell_id);
    assert_eq!(sell_order.status, OrderStatus::Filled);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_biome_mismatch_rejected() {
    let (env, _, client) = setup();
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    
    client.place_order(&seller, &OrderSide::Sell, &100, &1, &OrderRestriction::None, &Some(1), &None);
    client.place_order(&buyer, &OrderSide::Buy, &100, &1, &OrderRestriction::None, &Some(2), &None);
    
    client.match_orders(&2, &1);
}

#[test]
fn test_auto_match_multiple() {
    let (env, _admin, client) = setup();
    let seller1 = Address::generate(&env);
    let seller2 = Address::generate(&env);
    let buyer1 = Address::generate(&env);
    let buyer2 = Address::generate(&env);
    let (_bdc_id, bdc_client, _fv) = setup_full(&env, &client);
    
    for seller in [&seller1, &seller2] {
        for _ in 0..5 {
            mint_token(&env, &bdc_client, seller, 0);
        }
    }
    
    client.place_order(&seller1, &OrderSide::Sell, &90, &5, &OrderRestriction::None, &None, &None);
    client.place_order(&seller2, &OrderSide::Sell, &95, &5, &OrderRestriction::None, &None, &None);
    client.place_order(&buyer1, &OrderSide::Buy, &100, &5, &OrderRestriction::None, &None, &None);
    client.place_order(&buyer2, &OrderSide::Buy, &110, &5, &OrderRestriction::None, &None, &None);
    
    let count = client.auto_match();
    
    assert!(count > 0);
    for id in [1u64, 2, 3, 4] {
        let order = client.get_order(&id);
        assert_eq!(order.status, OrderStatus::Filled);
    }
}

#[test]
fn test_fee_deduction() {
    let (env, _admin, client) = setup();
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    let (_bdc_id, bdc_client, _fv) = setup_full(&env, &client);
    
    for _ in 0..10 {
        mint_token(&env, &bdc_client, &seller, 0);
    }
    
    let sell_id = client.place_order(&seller, &OrderSide::Sell, &100, &10, &OrderRestriction::None, &None, &None);
    let buy_id = client.place_order(&buyer, &OrderSide::Buy, &100, &10, &OrderRestriction::None, &None, &None);
    
    let (fill_qty, fill_price, fee) = client.match_orders(&buy_id, &sell_id);
    
    assert_eq!(fill_qty, 10);
    assert_eq!(fill_price, 100);
    // fee = 100 * 10 * 25 / 10000 = 2
    assert_eq!(fee, 2);
}

#[test]
fn test_fok_enforcement() {
    let (env, _admin, client) = setup();
    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    let (_bdc_id, bdc_client, _fv) = setup_full(&env, &client);
    
    for _ in 0..5 {
        mint_token(&env, &bdc_client, &seller, 0);
    }
    
    let buy_id = client.place_order(&buyer, &OrderSide::Buy, &100, &10, &OrderRestriction::FillOrKill, &None, &None);
    let sell_id = client.place_order(&seller, &OrderSide::Sell, &100, &5, &OrderRestriction::None, &None, &None);
    
    let (fill_qty, _, _) = client.match_orders(&buy_id, &sell_id);
    
    assert_eq!(fill_qty, 0);
    let buy_order = client.get_order(&buy_id);
    assert_eq!(buy_order.status, OrderStatus::Cancelled);
    let sell_order = client.get_order(&sell_id);
    assert_eq!(sell_order.status, OrderStatus::Open);
}
