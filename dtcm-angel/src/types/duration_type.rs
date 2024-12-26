/// Duration type
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DurationType {
    #[serde(rename = "DAY")]
    /// Regular Order
    Day,
    #[serde(rename = "IOC")]
    /// Immediate or Cancel
    Ioc,
}

impl Default for DurationType {
    fn default() -> Self {
        Self::Day
    }
}
