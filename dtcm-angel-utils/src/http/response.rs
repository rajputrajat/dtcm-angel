use super::error_codes::{deserialize_error_code, ErrorCodeOpt};
use crate::{UtilsError, UtilsResult};

/// Placeholder for response received from API calls
#[derive(Debug, Deserialize)]
pub struct Response<T> {
    /// Status for the API call
    pub status: bool,
    /// Message returned by the API
    pub message: String,
    /// Error code in case of any error
    #[serde(rename = "errorcode")]
    #[serde(deserialize_with = "deserialize_error_code")]
    pub error_code: ErrorCodeOpt,
    /// Data returned by the API
    pub data: Option<T>,
}

impl<T> Response<T> {
    /// Returns data from response
    pub fn into_data(self) -> UtilsResult<T> {
        self.data.ok_or_else(|| UtilsError::MissingData)
    }
}

impl<T> Response<Vec<T>> {
    /// Returns vector from response
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.data.unwrap_or_default()
    }
}
