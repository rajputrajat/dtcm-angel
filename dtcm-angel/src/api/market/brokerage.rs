use crate::types::{ExchangeType, ProductType, TransactionType};

/// Brokerage request type
#[derive(Debug, Serialize)]
#[api(POST, Brokerage)]
pub struct BrokerageReq {
    /// can include multiple orders in a single request
    pub orders: Vec<BrokeragePerProduct>,
}

/// Brokerage type per product
#[derive(Debug, Serialize)]
pub struct BrokeragePerProduct {
    /// type of product: `ProductType`
    pub product_type: ProductType,
    /// type of the transaction: `TransactionType`
    pub transaction_type: TransactionType,
    /// number of instruments being trade
    pub quantity: usize,
    /// price
    pub price: f64,
    /// on which exchange
    pub exchange: ExchangeType,
    /// name of the symbol being traded
    pub symbol_name: String,
    /// token
    pub token: String,
}

/// brokerage response type
#[derive(Debug, Deserialize)]
pub struct BrokerageResp {
    /// summary of brokerage
    pub summary: BrokerageSummary,
    /// brokerage summary per order
    pub charges: Vec<BrokerageSummary>,
}

/// an inner type of the brokerage response type
#[derive(Debug, Deserialize)]
pub struct BrokerageSummary {
    /// total charges for this order
    pub total_charges: f64,
    /// total traded value
    pub trade_value: f64,
    /// breakup of this category of brokerage
    pub breakup: Vec<BrokerageSubset>,
}

/// indicates a single type of brokerage subset
#[derive(Debug, Deserialize)]
pub struct BrokerageSubset {
    /// tax / brokerage name
    pub name: String,
    /// brokerage amount
    pub amount: f64,
    /// msg
    pub msg: String,
    /// further breakup
    pub breakup: Vec<BrokerageSubset>,
}
