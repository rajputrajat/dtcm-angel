use crate::types::ExchangeType;

/// nse, bse intraday scrips
#[derive(Debug, Serialize)]
#[api(POST, NseIntraday)]
pub struct NseIntradayScripReq;

/// nse, bse intraday scrips
#[derive(Debug, Serialize)]
#[api(POST, BseIntraday)]
pub struct BseIntradayScripReq;

/// Searched scrip
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IntradayScrip {
    /// Exchange name
    pub exchange: ExchangeType,
    /// symbol name
    pub symbol_name: String,
    /// multiplier
    pub multiplier: f64,
}

/// Searched scrip
#[derive(Debug, Deserialize)]
pub struct IntradayScripsRes(pub Vec<IntradayScrip>);
