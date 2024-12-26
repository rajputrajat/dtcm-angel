/// Product type
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ProductType {
    #[serde(rename = "DELIVERY")]
    /// Cash & Carry for equity (CNC)
    Delivery,
    #[serde(rename = "CARRYFORWARD")]
    /// Normal for futures and options (NRML)
    CarryForward,
    #[serde(rename = "MARGIN")]
    /// Margin Delivery
    Margin,
    #[serde(rename = "INTRADAY")]
    /// Margin Intraday Squareoff (MIS)
    IntraDay,
    #[serde(rename = "BO")]
    /// Bracket Order (Only for ROBO)
    Bo,
    #[serde(untagged)]
    /// unhandled values
    Unknown(String),
}

impl Default for ProductType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}
