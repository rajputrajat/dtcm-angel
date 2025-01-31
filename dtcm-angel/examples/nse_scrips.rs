use dtcm_angel::SmartConnect;

#[tokio::main]
async fn main() {
    env_logger::init();

    let api_key = dotenv::var("API_KEY").unwrap();
    let client_code = dotenv::var("CLIENT_CODE").unwrap();
    let pin = dotenv::var("PIN").unwrap();

    let sc = SmartConnect::new(api_key, client_code, pin).await.unwrap();
    let nse_scrips = sc.nse_intraday_scrips().await.unwrap();

    println!("{:?}", nse_scrips);
}
