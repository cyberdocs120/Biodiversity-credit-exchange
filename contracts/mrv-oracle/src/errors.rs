use soroban_sdk::contracterror;

#[derive(Copy, Clone, Debug, PartialEq)]
#[contracterror]
pub enum MrvOracleError {
    Unauthorized = 1,
    OracleAlreadyRegistered = 2,
    OracleNotFound = 3,
    ThresholdNotMet = 4,
    InvalidSignature = 5,
    InvalidSurveyData = 6,
    PolygonNotFound = 7,
    PolygonInactive = 8,
    SurveyNotFound = 9,
    SurveyAlreadyResolved = 10,
    DuplicateSurvey = 11,
    ContractPaused = 12,
}
