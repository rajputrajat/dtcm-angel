/// Transaction Types
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum TransactionType {
    #[serde(rename = "BUY")]
    /// Buy
    Buy,
    #[serde(rename = "SELL")]
    /// Sell
    Sell,
    #[serde(untagged)]
    /// unhandled values
    Unknown(String),
}

impl Default for TransactionType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}
