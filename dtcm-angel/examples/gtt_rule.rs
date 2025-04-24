use dtcm_angel::{
    SmartConnect,
    types::{ExchangeType, ProductType, TransactionType},
};

#[tokio::main]
async fn main() {
    let api_key = dotenv::var("API_KEY").unwrap();
    let client_code = dotenv::var("CLIENT_CODE").unwrap();
    let pin = dotenv::var("PIN").unwrap();
    let otp_token = dotenv::var("OTP_TOKEN").unwrap();

    let mut sc = SmartConnect::new(api_key, client_code, pin).await.unwrap();
    sc.generate_session(otp_token).await.unwrap();

    let mut order_req = SmartConnect::new_create_rule("SBIN-EQ", "3045");
    order_req = order_req
        .price(15)
        .qty(1)
        .trigger_price(10)
        .transaction_type(TransactionType::Buy)
        .product_type(ProductType::Delivery)
        .exchange(ExchangeType::NSE);

    // Create a GTT rulo
    let create_rule_resp = sc.create_rule(&order_req).await.unwrap();
    let rule_detail_req = SmartConnect::new_rule_detail(create_rule_resp.id);
    let rule_detail_resp = sc.rule_detail(&rule_detail_req).await.unwrap();
    println!("{:?}", rule_detail_resp);
}
