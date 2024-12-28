use std::str::FromStr;

use dtcm_angel_utils::ws::{IntoClientRequest, Request, WsStream};
use http_serde::http::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

use crate::order::OrderBook;

type Error = Box<dyn core::error::Error + Send + Sync>;

// Angel One Websocket URL
const ANGEL_WS_ORDER_STATUS_URL: &str = "wss://tns.angelone.in/smart-order-update";

/// Placeholder containing angel one web socket configuration
#[derive(Debug)]
pub struct AngelOneWsOrderStatus {
    /// Client code
    pub client_code: String,
    /// Feed token
    pub feed_token: String,
    /// auth token
    pub auth_token: String,
}

impl AngelOneWsOrderStatus {
    /// Returns a  new instance for the websocket
    pub fn new<C, F, A>(client_code: C, feed_token: F, auth_token: A) -> Self
    where
        C: Into<String>,
        F: Into<String>,
        A: Into<String>,
    {
        Self {
            client_code: client_code.into(),
            feed_token: feed_token.into(),
            auth_token: auth_token.into(),
        }
    }

    /// Prepares the websocket request with the required headers
    fn request(&self) -> Result<Request, Error> {
        let mut request = ANGEL_WS_ORDER_STATUS_URL.into_client_request()?;
        let headers = request.headers_mut();

        let client_code = self.client_code.parse()?;
        headers.insert("x-client-code", client_code);

        let feed_token = self.feed_token.parse()?;
        headers.insert("x-feed-token", feed_token);

        headers.insert(
            "Authorization",
            format!("Bearer {}", self.auth_token).parse()?,
        );

        Ok(request)
    }

    /// Returns the websocket stream
    pub async fn stream<M>(&self) -> Result<WsStream<M>, Error>
    where
        M: TryFrom<Vec<u8>, Error = Error> + DeserializeOwned,
    {
        Ok(WsStream::connect(self.request()?).await?)
    }
}

#[allow(missing_docs)]
#[derive(Debug, Deserialize, Clone)]
pub struct OrderStatus {
    #[serde(rename = "user-id")]
    pub user_id: String,
    #[serde(rename = "status-code")]
    pub status_code: StatusCode_,
    #[serde(rename = "order-status")]
    pub order_status: StatusEn,
    #[serde(rename = "error-message")]
    pub error_message: ErrorCode,
    #[serde(rename = "orderData")]
    pub order_data: OrderBook,
}

#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct StatusCode_(pub StatusCode);

impl<'de> Deserialize<'de> for StatusCode_ {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = String::deserialize(de)?;
        let status_code = StatusCode::from_str(&code).map_err(serde::de::Error::custom)?;
        Ok(Self(status_code))
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StatusEn {
    AfterSuccessfulConnection,
    Open,
    Cancelled,
    Rejected,
    Modified,
    Complete,
    AfterMarketOrderReqReceived,
    CancelledAfterMarketOrder,
    ModifyAfterMarketOrderReqReceived,
    OpenPending,
    TriggerPending,
    ModifyPending,
    Unknown(String),
}

impl<'de> Deserialize<'de> for StatusEn {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let variant = String::deserialize(de)?;
        Ok(match variant.as_str() {
            "AB00" => Self::AfterSuccessfulConnection,
            "AB01" => Self::Open,
            "AB02" => Self::Cancelled,
            "AB03" => Self::Rejected,
            "AB04" => Self::Modified,
            "AB05" => Self::Complete,
            "AB06" => Self::AfterMarketOrderReqReceived,
            "AB07" => Self::CancelledAfterMarketOrder,
            "AB08" => Self::ModifyAfterMarketOrderReqReceived,
            "AB09" => Self::OpenPending,
            "AB10" => Self::TriggerPending,
            "AB11" => Self::ModifyPending,
            _other => Self::Unknown(variant),
        })
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorCode {
    AuthorizationTokenInvalid,
    AuthorizationTokenExpired,
    ConnectionLimitBreached,
    NoError,
    Unknown(String),
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let variant = String::deserialize(de)?;
        Ok(match variant.as_str() {
            "" => Self::NoError,
            "401" => Self::AuthorizationTokenInvalid,
            "403" => Self::AuthorizationTokenExpired,
            "429" => Self::ConnectionLimitBreached,
            _other => Self::Unknown(variant),
        })
    }
}

impl TryFrom<&[u8]> for OrderStatus {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let ob: Self = serde_json::from_slice(value)?;
        Ok(ob)
    }
}

impl TryFrom<Vec<u8>> for OrderStatus {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from(value.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_code_deser() {
        assert_eq!(
            ErrorCode::AuthorizationTokenInvalid,
            serde_json::from_str(r#""401""#).unwrap()
        );
        assert_eq!(
            ErrorCode::Unknown(String::from("420")),
            serde_json::from_str(r#""420""#).unwrap()
        );
        assert_eq!(ErrorCode::NoError, serde_json::from_str(r#""""#).unwrap());
    }

    #[test]
    fn test_status_code_deser() {
        assert_eq!(
            StatusEn::AfterSuccessfulConnection,
            serde_json::from_str(r#""AB00""#).unwrap()
        );
        assert_eq!(
            StatusEn::ModifyPending,
            serde_json::from_str(r#""AB11""#).unwrap()
        );
        assert_eq!(
            StatusEn::Unknown("AB12".to_string()),
            serde_json::from_str(r#""AB12""#).unwrap()
        );
    }
}
