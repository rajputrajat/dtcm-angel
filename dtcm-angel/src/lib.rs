//! Crate contains Angel One API SDK data structures and calls
#![forbid(unsafe_code)]
#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(missing_docs)]
#![deny(clippy::all)]

#[macro_use]
extern crate dtcm_angel_derive;

#[macro_use]
extern crate serde;

use thiserror::Error as ThisError;

mod smart_connect;
pub use smart_connect::SmartConnect;

mod api;
pub use api::{funds, gtt, market, order, portfolio, user, ws};

/// Various types for Angel One API SDK
pub mod types;

/// the Error type from utils
#[derive(Debug, ThisError)]
pub enum Error {
    /// boxed error
    #[error("boxed error")]
    BoxedError(Box<dyn core::error::Error + Send + Sync>),
    /// errors from utils crate
    #[error(transparent)]
    UtilsError(#[from] dtcm_angel_utils::UtilsError),
    /// session not established
    #[error("unable to establish the session")]
    SessionEstablishmentError,
    /// subscription error
    #[error("Params required for the subscription request")]
    InvalidSubscriptionRequestParamsRequired,
    /// token required
    #[error("Token required for the token list")]
    InvalidSubscriptionRequestTokenRequired,
    /// invalid subscription exchange
    #[error("Invalid Subscription Exchange")]
    InvalidSubscriptionExchange,
    /// invalid subscription mode
    #[error("Invalid Subscription mode")]
    InvalidSubscriptionMode,
    /// interval error
    #[error("{0}")]
    IntervalError(String),
}

/// custom result type for crate
pub type Result<T> = std::result::Result<T, Error>;
