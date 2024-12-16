use std::time::Duration;

use tokio_stream::StreamExt;

use dtcm_angel::{
    ws::{AngelOneWsOrderStatus, OrderStatus},
    SmartConnect,
};

#[tokio::main]
async fn main() {
    env_logger::init();
    let api_key = dotenv::var("API_KEY").unwrap();
    let client_code = dotenv::var("CLIENT_CODE").unwrap();
    let pin = dotenv::var("PIN").unwrap();
    let otp_token = dotenv::var("OTP_TOKEN").unwrap();

    let mut sc = SmartConnect::new(api_key, &client_code, pin).await.unwrap();
    sc.generate_session(otp_token).await.unwrap();
    let feed_token = sc.current_feed_token().unwrap();

    let session = sc.session().unwrap();
    let ao_ws = AngelOneWsOrderStatus::new(client_code, feed_token, &session.jwt_token);

    let mut ws_stream = ao_ws.stream::<OrderStatus>().await.unwrap();

    let subscription_message = "";
    ws_stream.subscribe(subscription_message).await.unwrap();
    let mut timer = tokio::time::interval(Duration::from_secs(10));
    loop {
        tokio::select! {
            _ = timer.tick() => {
                ws_stream.subscribe("ping").await.unwrap();
            }
            Some(m) = ws_stream.next() => {
                println!("{:?}", m);
            }
        }
    }
}
