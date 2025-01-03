use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::UtilsResult;

use super::Response;

/// Implementation to return endpoint and url
pub trait Api {
    /// Returns the endpoint for the implemented object
    fn end_point() -> super::EndPoint;
    /// Returns the url for the implemented object
    fn url() -> String {
        Self::end_point().url()
    }
}

/// Implementation to make GET requests to the API
#[async_trait]
pub trait HttpFetcher: Api {
    /// Returns the fetched response with data
    async fn fetch<B>(http: &super::HttpClient, body: B) -> UtilsResult<super::Response<Self>>
    where
        B: serde::Serialize + Send + Sync,
        Self: DeserializeOwned + std::fmt::Debug,
    {
        Ok(http.get(Self::end_point(), &body).await?)
    }

    /// Returns the fetched data
    async fn fetch_data<B>(http: &super::HttpClient, body: B) -> UtilsResult<Self>
    where
        B: serde::Serialize + Send + Sync,
        Self: DeserializeOwned + std::fmt::Debug,
    {
        Self::fetch(http, body).await.and_then(Response::into_data)
    }

    /// Returns the fetched vector of data
    async fn fetch_vec<B>(http: &super::HttpClient, body: B) -> UtilsResult<Vec<Self>>
    where
        B: serde::Serialize + Send + Sync,
        Self: DeserializeOwned + std::fmt::Debug,
    {
        Ok(http
            .get(Self::end_point(), &body)
            .await
            .map(Response::into_vec)?)
    }
}

/// Implementation to make POST requests to the API
#[async_trait]
pub trait HttpSender: Api {
    /// Sends the data in body to the API and returns the Response
    async fn send<R>(&self, http: &super::HttpClient) -> UtilsResult<super::Response<R>>
    where
        R: DeserializeOwned + std::fmt::Debug,
        Self: serde::Serialize + std::fmt::Debug,
    {
        Ok(http.post(Self::end_point(), self).await?)
    }

    /// Sends the data in body to the API and returns the received data
    async fn send_data<R>(&self, http: &super::HttpClient) -> UtilsResult<R>
    where
        R: DeserializeOwned + std::fmt::Debug,
        Self: serde::Serialize + std::fmt::Debug,
    {
        self.send::<R>(http).await.and_then(Response::into_data)
    }

    /// Sends the data in body to the API and returns the received vector
    async fn send_vec<R>(&self, http: &super::HttpClient) -> UtilsResult<Vec<R>>
    where
        R: DeserializeOwned + std::fmt::Debug,
        Self: serde::Serialize + std::fmt::Debug,
    {
        self.send::<Vec<R>>(http)
            .await
            .and_then(Response::into_data)
    }
}
