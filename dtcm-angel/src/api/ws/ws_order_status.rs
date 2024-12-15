use dtcm_angel_utils::ws::{IntoClientRequest, Request, WsStream};
use serde::de::DeserializeOwned;

type Error = Box<dyn core::error::Error>;

// Angel One Websocket URL
const ANGEL_WS_ORDER_STATUS_URL: &str = "wss://tns.angelone.in/smart-order-update";

/// Placeholder containing angel one web socket configuration
#[derive(Debug)]
pub struct AngelOneWsOrderStatus {
    /// Client code
    pub client_code: String,
    /// Feed token
    pub feed_token: String,
}

impl AngelOneWsOrderStatus {
    /// Returns a  new instance for the websocket
    pub fn new<C, F>(client_code: C, feed_token: F) -> Self
    where
        C: Into<String>,
        F: Into<String>,
    {
        Self {
            client_code: client_code.into(),
            feed_token: feed_token.into(),
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
