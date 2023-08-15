use std::fmt;

#[derive(Debug)]
pub enum MPesaError {
    NetworkError(reqwest::Error),
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_validate_from_code() {
        assert_eq!(
            MPesaError::from_code(200, "Request processed successfully"),
            MPesaError::Successful("Request processed successfully".to_string())
        );
        assert_eq!(
            MPesaError::from_code(201, "Request processed successfully"),
            MPesaError::Successful("Request processed successfully".to_string())
        );
        assert_eq!(
            MPesaError::from_code(500, "Internal Error"),
            MPesaError::InternalError
        );
        assert_eq!(
            MPesaError::from_code(401, "INS-2"),
            MPesaError::InvalidAPIKey
        );
        assert_eq!(
            MPesaError::from_code(401, "INS-4"),
            MPesaError::UserNotActive
        );
        assert_eq!(
            MPesaError::from_code(401, "INS-5"),
            MPesaError::TransactionCancelledByCustomer
        );
        assert_eq!(
            MPesaError::from_code(401, "INS-6"),
            MPesaError::TransactionFailed
        );
        assert_eq!(
            MPesaError::from_code(401, "INS-26"),
            MPesaError::Unauthorized
        );
        assert_eq!(
            MPesaError::from_code(408, "Request Timeout"),
            MPesaError::RequestTimeout
        );
        assert_eq!(
            MPesaError::from_code(409, "Duplicate Transaction"),
            MPesaError::DuplicateTransaction
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-13"),
            MPesaError::InvalidShortcode
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-14"),
            MPesaError::InvalidReference
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-15"),
            MPesaError::InvalidAmount
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-17"),
            MPesaError::InvalidTransactionReferenceLength
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-18"),
            MPesaError::InvalidTransactionID
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-19"),
            MPesaError::InvalidThirdPartyReference
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-20"),
            MPesaError::MissingParameters
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-21"),
            MPesaError::ParameterValidationFailed
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-22"),
            MPesaError::InvalidOperationType
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-23"),
            MPesaError::UnknownStatus
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-24"),
            MPesaError::InvalidInitiatorIdentifier
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-25"),
            MPesaError::InvalidSecurityCredential
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-993"),
            MPesaError::DirectDebitMissing
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-994"),
            MPesaError::DirectDebitAlreadyExists
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-995"),
            MPesaError::CustomerProfileProblem
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-996"),
            MPesaError::CustomerAccountNotActive
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-997"),
            MPesaError::LinkingTransactionNotFound
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-998"),
            MPesaError::InvalidMarket
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-2001"),
            MPesaError::InitiatorAuthenticationError
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-2002"),
            MPesaError::ReceiverInvalid
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-2051"),
            MPesaError::MSISDNInvalid
        );
        assert_eq!(
            MPesaError::from_code(400, "INS-2057"),
            MPesaError::LanguageCodeInvalid
        );
        assert_eq!(
            MPesaError::from_code(503, "Service Unavailable"),
            MPesaError::TemporaryOverload
        );
        assert_eq!(
            MPesaError::from_code(422, "Insufficient Balance"),
            MPesaError::InsufficientBalance
        );
        assert_eq!(
            MPesaError::from_code(999, "Unknown Error"),
            MPesaError::Other(999, "Unknown Error".to_string())
        );
    }
}

