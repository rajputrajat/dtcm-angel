use reqwest::{Client, ClientBuilder, IntoUrl, Method, StatusCode, redirect::Policy};
use serde::{Serialize, de::DeserializeOwned};

use super::{EndPoint, HttpHeader, Response};
use crate::{UtilsError, UtilsResult};

/// Placeholder for the Http client
#[derive(Debug)]
pub struct HttpClient {
    /// Inner client
    client: Client,
    /// JWT token for bearer
    jwt_token: Option<String>,
}

impl HttpClient {
    /// Returns a new instance for the http client
    pub async fn new<A>(api_key: A) -> UtilsResult<Self>
    where
        A: AsRef<str>,
    {
        let http_headers = HttpHeader::new(api_key.as_ref(), None::<String>).await?;

        let client = ClientBuilder::new()
            .redirect(Policy::custom(|a| a.follow()))
            .default_headers(http_headers.into_inner())
            .build()
            .map_err(|e| {
                error!("Failed to create http client {e}");
                e
            })?;

        Ok(Self {
            client,
            jwt_token: None,
        })
    }

    /// Sets the jwt token for authorization header
    pub fn jwt_token<J>(&mut self, jwt_token: J)
    where
        J: Into<String>,
    {
        self.jwt_token = Some(jwt_token.into());
    }

    /// Makes the http request
    pub async fn request<B, R>(
        &self,
        method: Method,
        ep: EndPoint,
        body: &B,
    ) -> UtilsResult<Response<R>>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned + std::fmt::Debug,
    {
        debug!("New {method} request for {ep}");

        let request = match method {
            Method::GET => self.client.get(ep.url()).query(body),
            Method::POST => self.client.post(ep.url()).json(body),
            _ => unimplemented!(),
        };

        let request = match self.jwt_token.as_ref() {
            Some(token) => request.bearer_auth(token),
            None => request,
        };
        trace!("request: {request:?}");

        let req_res = request.send().await.map_err(|e| {
            error!("{method} request to {ep} failed: {e:?}");
            e
        })?;
        trace!("response: {req_res:?}");

        if req_res.status()
            == StatusCode::from_u16(403)
                .map_err(|_e| UtilsError::InvalidStatusCode(req_res.status().as_u16()))?
        {
            return Err(UtilsError::RateLimitExceeded);
        }

        let res: Response<R> = req_res.json().await.map_err(|e| {
            error!("endpoint: {ep}, error: {e:?}");
            e
        })?;

        if !res.status {
            error!("{method} request to {ep} failed: {}", res.message);
            return Err(UtilsError::FailedRequest(res.message));
        }

        debug!("{method} request to {ep} completed");

        Ok(res)
    }
    /// Makes the get request
    pub async fn get_json_url<U, R>(url: U) -> UtilsResult<R>
    where
        U: IntoUrl,
        R: DeserializeOwned + std::fmt::Debug,
    {
        let res = reqwest::get(url).await?.json().await?;
        Ok(res)
    }

    /// Makes the get request
    pub async fn get<B, R>(&self, ep: EndPoint, body: &B) -> UtilsResult<Response<R>>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned + std::fmt::Debug,
    {
        self.request(Method::GET, ep, body).await
    }

    /// Makes the post request
    pub async fn post<B, R>(&self, ep: EndPoint, body: &B) -> UtilsResult<Response<R>>
    where
        B: Serialize + ?Sized,
        R: DeserializeOwned + std::fmt::Debug,
    {
        self.request(Method::POST, ep, body).await
    }
}
