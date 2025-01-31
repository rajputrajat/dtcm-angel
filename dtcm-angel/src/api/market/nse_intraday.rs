use crate::types::ExchangeType;

/// Searched scrip
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[api(GET, NseIntraday)]
pub struct IntradayScrip {
    /// Exchange name
    pub exchange: ExchangeType,
    /// symbol name
    pub symbol_name: String,
    /// multiplier
    pub multiplier: f64,
}
