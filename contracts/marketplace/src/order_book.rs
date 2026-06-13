use soroban_sdk::{Env, Vec};
use crate::types::{Order, OrderSide, OrderStatus};
use crate::storage::{read_order, read_order_counter};

pub fn buy_orders(env: &Env) -> Vec<Order> {
    let mut orders = Vec::new(env);
    let counter = read_order_counter(env);
    
    for i in 1..=counter {
        if let Some(order) = read_order(env, i) {
            if order.side == OrderSide::Buy && order.status == OrderStatus::Open && order.remaining_qty > 0 {
                orders.push_back(order);
            }
        }
    }
    
    sort_orders(env, orders, OrderSide::Buy)
}

pub fn sell_orders(env: &Env) -> Vec<Order> {
    let mut orders = Vec::new(env);
    let counter = read_order_counter(env);
    
    for i in 1..=counter {
        if let Some(order) = read_order(env, i) {
            if order.side == OrderSide::Sell && order.status == OrderStatus::Open && order.remaining_qty > 0 {
                orders.push_back(order);
            }
        }
    }
    
    sort_orders(env, orders, OrderSide::Sell)
}

fn sort_orders(_env: &Env, mut orders: Vec<Order>, side: OrderSide) -> Vec<Order> {
    if orders.len() < 2 {
        return orders;
    }
    
    // Bubble sort for simplicity in MVP, as suggested in plan.md
    let n = orders.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            let o1 = orders.get(j).unwrap();
            let o2 = orders.get(j + 1).unwrap();
            
            let should_swap = match side {
                OrderSide::Buy => {
                    if o1.price < o2.price {
                        true
                    } else if o1.price == o2.price {
                        o1.timestamp > o2.timestamp
                    } else {
                        false
                    }
                },
                OrderSide::Sell => {
                    if o1.price > o2.price {
                        true
                    } else if o1.price == o2.price {
                        o1.timestamp > o2.timestamp
                    } else {
                        false
                    }
                }
            };
            
            if should_swap {
                orders.set(j, o2);
                orders.set(j + 1, o1);
            }
        }
    }
    orders
}

pub fn best_bid(env: &Env) -> Option<Order> {
    let orders = buy_orders(env);
    if orders.is_empty() {
        None
    } else {
        Some(orders.get(0).unwrap())
    }
}

pub fn best_ask(env: &Env) -> Option<Order> {
    let orders = sell_orders(env);
    if orders.is_empty() {
        None
    } else {
        Some(orders.get(0).unwrap())
    }
}

pub fn best_bid_for_biome(env: &Env, biome: u32) -> Option<Order> {
    let orders = buy_orders(env);
    for i in 0..orders.len() {
        let order = orders.get(i).unwrap();
        if let Some(b) = order.biome_filter {
            if b == biome { return Some(order); }
        } else {
            return Some(order);
        }
    }
    None
}

pub fn best_ask_for_biome(env: &Env, biome: u32) -> Option<Order> {
    let orders = sell_orders(env);
    for i in 0..orders.len() {
        let order = orders.get(i).unwrap();
        if let Some(b) = order.biome_filter {
            if b == biome { return Some(order); }
        } else {
            return Some(order);
        }
    }
    None
}
