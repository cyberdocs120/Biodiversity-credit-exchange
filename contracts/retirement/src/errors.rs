use soroban_sdk::contracterror;

#[derive(Copy, Clone, Debug, PartialEq)]
#[contracterror]
pub enum RetirementError {
    Unauthorized = 1,
    ReceiptNotFound = 2,
    TokenAlreadyRetired = 3,
    InvalidToken = 4,
    PolygonMismatch = 5,
    EmptyTokenList = 6,
}
