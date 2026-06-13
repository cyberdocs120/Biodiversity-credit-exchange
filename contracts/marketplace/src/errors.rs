use soroban_sdk::contracterror;

#[derive(Copy, Clone, Debug, PartialEq)]
#[contracterror]
pub enum MarketError {
    Unauthorized = 1,
    OrderNotFound = 2,
    OrderFilled = 3,
    PriceMismatch = 4,
    InsufficientBalance = 5,
    FeeCapExceeded = 6,
    BiomeMismatch = 7,
    VintageMismatch = 8,
    InvalidQuantity = 9,
}
