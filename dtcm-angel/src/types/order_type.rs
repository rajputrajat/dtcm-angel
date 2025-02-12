/// Order type
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum OrderType {
    #[serde(rename = "MARKET")]
    /// Market Order(MKT)
    Market,
    #[serde(rename = "LIMIT")]
    /// Limit Order(L)
    Limit,
    #[serde(rename = "STOPLOSS_LIMIT")]
    /// Stop Loss Limit Order(SL)
    StopLossLimit,
    #[serde(rename = "STOPLOSS_MARKET")]
    /// Stop Loss Market Order(SL-M)
    StopLossMarket,
    #[serde(untagged)]
    /// unhandled values
    Unknown(String),
}

impl Default for OrderType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}
