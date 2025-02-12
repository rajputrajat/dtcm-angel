/// Exchange Type
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum ExchangeType {
    /// BSE Equity
    BSE,
    /// NSE Equity
    NSE,
    /// NSE Future and Options
    NFO,
    /// MCX Commodity
    MCX,
    /// BSE Futures and Options
    BFO,
    /// Currency Derivate Segment
    CDS,
    /// National Commodity and Derivatives Exchange
    NCDEX,
    /// not sure what NCO is, but it's part of instruments
    NCO,
    #[serde(untagged)]
    /// unhandled values
    Unknown(String),
}

impl Default for ExchangeType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}

/// Exchange type for market data requests
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MarketDataExchange {
    /// NSE Equity
    NSE,
    /// NSE Future and Options
    NFO,
}

impl Default for MarketDataExchange {
    fn default() -> Self {
        Self::NSE
    }
}
