/// Order Variety
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum OrderVariety {
    /// Normal Order (Regular)
    #[serde(rename = "NORMAL")]
    Normal,
    /// Stop loss order
    #[serde(rename = "STOPLOSS")]
    StopLoss,
    /// After Market Order
    #[serde(rename = "AMO")]
    Amo,
    #[serde(rename = "ROBO")]
    /// ROBO (Bracket Order)
    Robo,
    #[serde(untagged)]
    /// unhandled values
    Unknown(String),
}

impl Default for OrderVariety {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}
