use soroban_sdk::contracterror;

#[derive(Copy, Clone, Debug, PartialEq)]
#[contracterror]
pub enum BdcTokenError {
    Unauthorized = 1,
    BdcAlreadyRetired = 2,
    InsufficientBalance = 3,
    TokenNotFound = 4,
    PolygonNotFound = 5,
    DuplicateMint = 6,
}
