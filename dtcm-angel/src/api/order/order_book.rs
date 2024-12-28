use serde::Deserialize;

use crate::types::{
    DurationType, ExchangeType, OrderType, OrderVariety, ProductType, TransactionType,
};

/// Placeholder for the order book
#[allow(missing_docs)]
#[derive(Debug, Deserialize, Default, PartialEq, Clone)]
#[api(GET, OrderBook)]
pub struct OrderBook {
    pub variety: OrderVariety,
    #[serde(rename = "ordertype")]
    pub order_type: OrderType,
    #[serde(rename = "producttype")]
    pub product_type: ProductType,
    pub duration: DurationType,
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
    #[serde(rename = "transactiontype")]
    pub transaction_type: TransactionType,
    pub exchange: ExchangeType,
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_empty_orderbook() {
        let book: OrderBook = serde_json::from_str(EMPTY_BOOK).unwrap();
        assert_eq!(book, OrderBook::default());
    }

    #[test]
    fn test_parse_normal_orderbook() {
        let parsed_book: OrderBook = serde_json::from_str(NORMAL_ORDER).unwrap();

        let mut book = OrderBook::default();
        book.variety = OrderVariety::Normal;
        book.order_type = OrderType::Market;
        book.product_type = ProductType::Delivery;
        book.duration = DurationType::Day;
        book.quantity = "1".to_string();
        book.disclosed_quantity = "0".to_string();
        book.trading_symbol = "MOM30IETF-EQ".to_string();
        book.transaction_type = TransactionType::Buy;
        book.exchange = ExchangeType::NSE;
        book.symbol_token = "10585".to_string();
        book.strike_price = -1.0;
        book.lot_size = "1".to_string();
        book.cancel_size = "0".to_string();
        book.filled_shares = "0".to_string();
        book.unfilled_shares = "1".to_string();
        book.order_id = "241226001121984".to_string();
        book.status = "open".to_string();
        book.order_status = "open".to_string();
        book.update_time = "26-Dec-2024 15:55:01".to_string();
        book.exch_time = "26-Dec-2024 15:55:01".to_string();
        book.exch_order_update_time = "26-Dec-2024 15:55:01".to_string();

        assert_eq!(parsed_book, book);
    }

    const EMPTY_BOOK: &str = r#"{
        "variety": "",
        "ordertype": "",
        "ordertag": "",
        "producttype": "",
        "price": 0,
        "triggerprice": 0,
        "quantity": "",
        "disclosedquantity": "",
        "duration": "",
        "squareoff": 0,
        "stoploss": 0,
        "trailingstoploss": 0,
        "tradingsymbol": "",
        "transactiontype": "",
        "exchange": "",
        "symboltoken": "",
        "instrumenttype": "",
        "strikeprice": 0,
        "optiontype": "",
        "expirydate": "",
        "lotsize": "",
        "cancelsize": "",
        "averageprice": 0,
        "filledshares": "",
        "unfilledshares": "",
        "orderid": "",
        "text": "",
        "status": "",
        "orderstatus": "",
        "updatetime": "",
        "exchtime": "",
        "exchorderupdatetime": "",
        "fillid": "",
        "filltime": "",
        "parentorderid": ""
     }"#;

    const NORMAL_ORDER: &str = r#"{
         "variety": "NORMAL",
         "ordertype": "MARKET",
         "ordertag": "",
         "producttype": "DELIVERY",
         "price": 0.0,
         "triggerprice": 0.0,
         "quantity": "1",
         "disclosedquantity": "0",
         "duration": "DAY",
         "squareoff": 0.0,
         "stoploss": 0.0,
         "trailingstoploss": 0.0,
         "tradingsymbol": "MOM30IETF-EQ",
         "transactiontype": "BUY",
         "exchange": "NSE",
         "symboltoken": "10585",
         "instrumenttype": "",
         "strikeprice": -1.0,
         "optiontype": "",
         "expirydate": "",
         "lotsize": "1",
         "cancelsize": "0",
         "averageprice": 0.0,
         "filledshares": "0",
         "unfilledshares": "1",
         "orderid": "241226001121984",
         "text": "",
         "status": "open",
         "orderstatus": "open",
         "updatetime": "26-Dec-2024 15:55:01",
         "exchtime": "26-Dec-2024 15:55:01",
         "exchorderupdatetime": "26-Dec-2024 15:55:01",
         "fillid": "",
         "filltime": "",
         "parentorderid": ""
     }"#;
}
