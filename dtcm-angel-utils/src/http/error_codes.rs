use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;
use thiserror::Error as ThisError;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[repr(u8)] // for error serial-numbers from codes api page
/// error codes for smartapi
pub enum ErrorCode_ {
    /// #[strum(message = "InvalidToken")]
    AG8001 = 1,
    /// #[strum(message = "TokenExpired")]
    AG8002,
    /// #[strum(message = "TokenMissing")]
    AG8003,
    /// #[strum(message = "InvalidRefreshToken")]
    AB8050,
    /// #[strum(message = "RefreshTokenExpired")]
    AB8051,
    /// #[strum(message = "InvalidEmailOrPassword")]
    AB1000,
    /// #[strum(message = "InvalidEmail")]
    AB1001,
    /// #[strum(message = "InvalidPasswordLength")]
    AB1002,
    /// #[strum(message = "ClientAlreadyExists")]
    AB1003,
    /// #[strum(message = "SomethingWentWrongTryAfterSometime")]
    AB1004,
    /// #[strum(message = "UserTypeMustBeUser")]
    AB1005,
    /// #[strum(message = "ClientIsBlockForTrading")]
    AB1006,
    /// #[strum(message = "AmxError")]
    AB1007,
    /// #[strum(message = "InvalidOrderVariety")]
    AB1008,
    /// #[strum(message = "SymbolNotFound")]
    AB1009,
    /// #[strum(message = "AmxSessionExpired")]
    AB1010,
    /// #[strum(message = "ClientNotLogin")]
    AB1011,
    /// #[strum(message = "InvalidProductType")]
    AB1012,
    /// #[strum(message = "OrderNotFound")]
    AB1013,
    /// #[strum(message = "TradeNotFound")]
    AB1014,
    /// #[strum(message = "HoldingNotFound")]
    AB1015,
    /// #[strum(message = "PositionNotFound")]
    AB1016,
    /// #[strum(message = "PositionConversionFailed")]
    AB1017,
    /// #[strum(message = "FailedToGetSymbolDetails")]
    AB1018,
    /// #[strum(message = "ErrorNotSpecified")]
    AB2000,
    /// #[strum(message = "InternalErrorPleaseTryAfterSometime")]
    AB2001,
    /// #[strum(message = "OldPasswordMismatch")]
    AB1031,
    /// #[strum(message = "UserNotFound")]
    AB1032,
    /// #[strum(message = "RoboOrderIsBlock")]
    AB2002,
}

/// error codes for smartapi
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub enum ErrorCode {
    /// code
    Code(ErrorCode_),
    /// err
    Err(String),
}

/// the error
#[derive(Debug, ThisError)]
pub enum ErrorCodesError {
    #[error("invalid error code")]
    /// invalid error
    InvalidErrorCode(String),
}

/// error codes
#[derive(Debug, Clone)]
pub struct ErrorCodeOpt(pub Option<ErrorCode>);

impl FromStr for ErrorCodeOpt {
    type Err = ErrorCodesError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AG8001" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AG8001)))),
            "AG8002" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AG8002)))),
            "AG8003" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AG8003)))),
            "AB8050" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB8050)))),
            "AB8051" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB8051)))),
            "AB1000" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1000)))),
            "AB1001" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1001)))),
            "AB1002" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1002)))),
            "AB1003" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1003)))),
            "AB1004" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1004)))),
            "AB1005" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1005)))),
            "AB1006" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1006)))),
            "AB1007" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1007)))),
            "AB1008" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1008)))),
            "AB1009" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1009)))),
            "AB1010" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1010)))),
            "AB1011" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1011)))),
            "AB1012" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1012)))),
            "AB1013" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1013)))),
            "AB1014" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1014)))),
            "AB1015" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1015)))),
            "AB1016" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1016)))),
            "AB1017" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1017)))),
            "AB1018" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1018)))),
            "AB2000" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB2000)))),
            "AB2001" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB2001)))),
            "AB2002" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB2002)))),
            "AB1031" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1031)))),
            "AB1032" => Ok(Self(Some(ErrorCode::Code(ErrorCode_::AB1032)))),
            "" => Ok(Self(None)),
            e => Ok(Self(Some(ErrorCode::Err(e.to_owned())))),
        }
    }
}

pub fn deserialize_error_code<'de, D>(deserializer: D) -> Result<ErrorCodeOpt, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    Ok(value.parse().unwrap())
}
