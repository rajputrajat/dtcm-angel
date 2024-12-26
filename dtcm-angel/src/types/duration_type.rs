/// Duration type
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DurationType {
    #[serde(rename = "DAY")]
    /// Regular Order
    Day,
    #[serde(rename = "IOC")]
    /// Immediate or Cancel
    Ioc,
    #[serde(untagged)]
    /// unhandled values
    Unknown(String),
}

impl Default for DurationType {
    fn default() -> Self {
        Self::Unknown(String::new())
    }
}
