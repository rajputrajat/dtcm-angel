//! Crate contains utility calls for the SDK, usable types are reexported from dtcm-angel crate
#![forbid(unsafe_code)]
#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(missing_docs)]
#![deny(clippy::all)]

#[macro_use]
extern crate pin_project;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde;

use mac_address::MacAddressError;
use std::{io, net::AddrParseError, string::FromUtf8Error, time::SystemTimeError};
use thiserror::Error as ThisError;
use tokio_tungstenite::tungstenite::http::header::InvalidHeaderValue;
use totp_rs::{SecretParseError, TotpUrlError};

/// Contains date related functionality
pub mod date;
/// Contains http related functionality
pub mod http;
/// Contains system related functionality
pub mod sys;
/// Contains websocket related functionality
pub mod ws;

/// the Error type from utils
#[derive(Debug, ThisError)]
pub enum UtilsError {
    /// chrono parser failed
    #[error(transparent)]
    ChronoParseError(#[from] chrono::ParseError),
    /// from reqwest
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    /// server rejected the request
    #[error("failed request from server")]
    FailedRequest(String),
    /// rate limit exceeded
    #[error("rate limit exceeded")]
    RateLimitExceeded,
    /// invalid status-code
    #[error("invalid status code: {0}")]
    InvalidStatusCode(u16),
    /// public ip count not be determined
    #[error(transparent)]
    IoError(#[from] io::Error),
    /// address parsing failed
    #[error(transparent)]
    AddrParsingFailed(#[from] AddrParseError),
    /// text parsing failed
    #[error(transparent)]
    Utf8ParsingFailed(#[from] FromUtf8Error),
    /// missing data in the api response
    #[error("missing data in API response")]
    MissingData,
    /// secret parse error
    #[error(transparent)]
    SecretParseFailed(#[from] SecretParseError),
    /// totp url error
    #[error(transparent)]
    TotpUrlError(#[from] TotpUrlError),
    /// system time error
    #[error(transparent)]
    SystemTimeError(#[from] SystemTimeError),
    /// local ip error
    #[error(transparent)]
    LocalIpError(#[from] local_ip_address::Error),
    /// mac address error
    #[error(transparent)]
    MacAddressError(#[from] MacAddressError),
    /// mac address error
    #[error("empty mac-address")]
    MacAddressNone,
    /// error from tungsnite crate
    #[error(transparent)]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
    /// error from tungsnite crate
    #[error("tunstenite: close frame requested")]
    TungsteniteCloseFrameError,
    /// serde failed
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    /// invalid header
    #[error(transparent)]
    InvalidHeader(#[from] InvalidHeaderValue),
    /// best 5 data failed   
    #[error("Invalid best five data flag")]
    InvalidBestFiveData,
}

/// custom result type for crate
pub type Result<T> = std::result::Result<T, UtilsError>;
