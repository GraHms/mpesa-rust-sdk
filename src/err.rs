use std::fmt;

#[derive(Debug)]
pub enum MPesaError {
    NetworkError(reqwest::Error),
    SerializationError(serde_json::Error),
    Successful(String),
    InternalError,
    InvalidAPIKey,
    UserNotActive,
    TransactionCancelledByCustomer,
    TransactionFailed,
    RequestTimeout,
    DuplicateTransaction,
    InvalidShortcode,
    InvalidReference,
    InvalidAmount,
    TemporaryOverload,
    InvalidTransactionReferenceLength,
    InvalidTransactionID,
    InvalidThirdPartyReference,
    MissingParameters,
    ParameterValidationFailed,
    InvalidOperationType,
    UnknownStatus,
    InvalidInitiatorIdentifier,
    InvalidSecurityCredential,
    Unauthorized,
    DirectDebitMissing,
    DirectDebitAlreadyExists,
    CustomerProfileProblem,
    CustomerAccountNotActive,
    LinkingTransactionNotFound,
    InvalidMarket,
    InitiatorAuthenticationError,
    ReceiverInvalid,
    InsufficientBalance,
    MSISDNInvalid,
    LanguageCodeInvalid,
    Other(u16, String),
}
impl From<reqwest::Error> for MPesaError {
    fn from(error: reqwest::Error) -> Self {
        MPesaError::NetworkError(error)
    }
}
impl From<serde_json::Error> for MPesaError {
    fn from(error: serde_json::Error) -> Self {
        MPesaError::SerializationError(error)
    }
}



impl MPesaError {

    pub fn from_code(code: u16, description: &str) -> Self {
        match code {
            200 | 201 => MPesaError::Successful(description.to_string()),
            500 => MPesaError::InternalError,
            401 => match description {
                "INS-2" => MPesaError::InvalidAPIKey,
                "INS-4" => MPesaError::UserNotActive,
                "INS-5" => MPesaError::TransactionCancelledByCustomer,
                "INS-6" => MPesaError::TransactionFailed,
                "INS-26" => MPesaError::Unauthorized,
                _ => MPesaError::Other(code, description.to_string()),
            },
            408 => MPesaError::RequestTimeout,
            409 => MPesaError::DuplicateTransaction,
            400 => match description {
                "INS-13" => MPesaError::InvalidShortcode,
                "INS-14" => MPesaError::InvalidReference,
                "INS-15" => MPesaError::InvalidAmount,
                "INS-17" => MPesaError::InvalidTransactionReferenceLength,
                "INS-18" => MPesaError::InvalidTransactionID,
                "INS-19" => MPesaError::InvalidThirdPartyReference,
                "INS-20" => MPesaError::MissingParameters,
                "INS-21" => MPesaError::ParameterValidationFailed,
                "INS-22" => MPesaError::InvalidOperationType,
                "INS-23" => MPesaError::UnknownStatus,
                "INS-24" => MPesaError::InvalidInitiatorIdentifier,
                "INS-25" => MPesaError::InvalidSecurityCredential,
                "INS-993" => MPesaError::DirectDebitMissing,
                "INS-994" => MPesaError::DirectDebitAlreadyExists,
                "INS-995" => MPesaError::CustomerProfileProblem,
                "INS-996" => MPesaError::CustomerAccountNotActive,
                "INS-997" => MPesaError::LinkingTransactionNotFound,
                "INS-998" => MPesaError::InvalidMarket,
                "INS-2001" => MPesaError::InitiatorAuthenticationError,
                "INS-2002" => MPesaError::ReceiverInvalid,
                "INS-2051" => MPesaError::MSISDNInvalid,
                "INS-2057" => MPesaError::LanguageCodeInvalid,
                _ => MPesaError::Other(code, description.to_string()),
            },
            503 => MPesaError::TemporaryOverload,
            422 => MPesaError::InsufficientBalance,
            _ => MPesaError::Other(code, description.to_string()),
        }
    }
}

impl fmt::Display for MPesaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for MPesaError {}

