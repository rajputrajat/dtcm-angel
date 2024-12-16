use serde::Deserialize;

use crate::types::{
    DurationType, ExchangeType, OrderType, OrderVariety, ProductType, TransactionType,
};

/// Placeholder for the order book
#[allow(missing_docs)]
#[derive(Debug, Deserialize)]
#[api(GET, OrderBook)]
pub struct OrderBook {
    #[serde(deserialize_with = "deserialize_empty")]
    pub variety: Option<OrderVariety>,
    #[serde(rename = "ordertype", deserialize_with = "deserialize_empty")]
    pub order_type: Option<OrderType>,
    #[serde(rename = "producttype", deserialize_with = "deserialize_empty")]
    pub product_type: Option<ProductType>,
    #[serde(deserialize_with = "deserialize_empty")]
    pub duration: Option<DurationType>,
    pub price: f64,
    #[serde(rename = "triggerprice")]
    pub trigger_price: f64,
    pub quantity: String,
    #[serde(rename = "disclosedquantity")]
    pub disclosed_quantity: String,
    #[serde(rename = "squareoff")]
    pub square_off: f64,
    #[serde(rename = "stoploss")]
    pub stop_loss: f64,
    #[serde(rename = "trailingstoploss")]
    pub trailing_stop_loss: f64,
    #[serde(rename = "tradingsymbol")]
    pub trading_symbol: String,
    #[serde(rename = "transactiontype", deserialize_with = "deserialize_empty")]
    pub transaction_type: Option<TransactionType>,
    #[serde(deserialize_with = "deserialize_empty")]
    pub exchange: Option<ExchangeType>,
    #[serde(rename = "symboltoken")]
    pub symbol_token: String,
    #[serde(rename = "ordertag")]
    pub order_tag: String,
    #[serde(rename = "instrumenttype")]
    pub instrument_type: String,
    #[serde(rename = "strikeprice")]
    pub strike_price: f64,
    #[serde(rename = "optiontype")]
    pub option_type: String,
    #[serde(rename = "expirydate")]
    pub expiry_date: String,
    #[serde(rename = "lotsize")]
    pub lot_size: String,
    #[serde(rename = "cancelsize")]
    pub cancel_size: String,
    #[serde(rename = "averageprice")]
    pub average_price: f64,
    #[serde(rename = "filledshares")]
    pub filled_shares: String,
    #[serde(rename = "unfilledshares")]
    pub unfilled_shares: String,
    #[serde(rename = "orderid")]
    pub order_id: String,
    #[serde(rename = "uniqueorderid")]
    pub unique_order_id: Option<String>,
    pub text: String,
    pub status: String,
    #[serde(rename = "orderstatus")]
    pub order_status: String,
    #[serde(rename = "updatetime")]
    pub update_time: String,
    #[serde(rename = "exchtime")]
    pub exch_time: String,
    #[serde(rename = "exchorderupdatetime")]
    pub exch_order_update_time: String,
    #[serde(rename = "fillid")]
    pub fill_id: String,
    #[serde(rename = "filltime")]
    pub fill_time: String,
    #[serde(rename = "parentorderid")]
    pub parent_order_id: String,
}

fn deserialize_empty<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None) // Treat empty string as None
    } else {
        let out = serde_json::from_str(&s).unwrap();
        Ok(Some(out))
    }
}
