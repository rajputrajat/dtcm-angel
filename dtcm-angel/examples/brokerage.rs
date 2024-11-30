use dtcm_angel::{
    market::{BrokeragePerProduct, BrokerageReq},
    types::{ExchangeType, ProductType, TransactionType},
    SmartConnect,
};

#[tokio::main]
async fn main() {
    let api_key = dotenv::var("API_KEY").unwrap();
    let client_code = dotenv::var("CLIENT_CODE").unwrap();
    let pin = dotenv::var("PIN").unwrap();
    let otp_token = dotenv::var("OTP_TOKEN").unwrap();

    let mut sc = SmartConnect::new(api_key, client_code, pin).await.unwrap();
    sc.generate_session(otp_token).await.unwrap();

    let brokerage = sc
        .brokerage(BrokerageReq {
            orders: vec![BrokeragePerProduct {
                product_type: ProductType::Delivery,
                transaction_type: TransactionType::Buy,
                quantity: 100,
                price: 100.,
                exchange: ExchangeType::NSE,
                symbol_name: "TRENT".to_string(),
                token: "1964".to_string(),
            }],
        })
        .await
        .unwrap();
    println!("{:?}", brokerage);
}
