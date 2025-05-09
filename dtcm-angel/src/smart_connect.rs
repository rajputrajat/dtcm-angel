use dtcm_angel_utils::{
    UtilsError,
    http::{HttpClient, HttpFetcher, HttpSender, INSTRUMENT_URL},
};
use log::{debug, error, trace};
use std::{collections::HashMap, error::Error as StdError, fmt::Display};

use crate::{
    Error, Result,
    funds::{MarginCalculatorPosition, MarginCalculatorReq, MarginCalculatorRes, Rms},
    gtt::{
        CancelRuleReq, CancelRuleRes, CreateRuleReq, CreateRuleRes, ModifyRuleReq, ModifyRuleRes,
        RuleDetailReq, RuleDetailRes, RuleListReq, RuleListRes,
    },
    market::{
        BrokerageReq, BrokerageResp, CandleDataReq, CandleDataRes, Instrument, IntradayScrip,
        LtpDataReq, LtpDataRes, MarketDataReq, MarketDataRes, SearchScripReq, SearchScripRes,
    },
    order::{
        CancelOrderReq, CancelOrderRes, IndividualOrderStatus, ModifyOrderReq, ModifyOrderRes,
        OrderBook, PlaceOrderReq, PlaceOrderRes, TradeBook,
    },
    portfolio::{AllHoldings, ConvertPositionReq, Holding, Position},
    types::{
        ExchangeType, Interval, MarketDataExchange, MarketMode, OrderVariety, ProductType,
        RuleType, TransactionType,
    },
    user::{LogoutReq, Profile, SessionReq, SessionRes, TokenReq},
};

/// Smart connect client to interact with Angel One API
#[derive(Debug)]
pub struct SmartConnect {
    /// API key from Angel One
    pub api_key: String,
    /// Client Code
    pub client_code: String,
    /// PIN for the client code
    pub pin: String,
    /// Session received
    pub session: Option<SessionRes>,
    /// User profile
    pub user: Option<Profile>,
    /// Http client to make requests
    pub http: HttpClient,
}

impl SmartConnect {
    /// Returns a new instance for the smart connect API
    pub async fn new<A, C, P>(api_key: A, client_code: C, pin: P) -> Result<Self>
    where
        A: Into<String>,
        C: Into<String>,
        P: Into<String>,
    {
        let api_key: String = api_key.into();

        let http = HttpClient::new(&api_key).await?;

        Ok(Self {
            api_key,
            client_code: client_code.into(),
            pin: pin.into(),
            session: None,
            user: None,
            http,
        })
    }

    /// Returns the available instruments
    pub async fn instruments() -> Result<Vec<Instrument>> {
        Ok(HttpClient::get_json_url(INSTRUMENT_URL).await?)
    }

    /// Generates the session to receive authentication tokens and user information
    pub async fn generate_session<O>(&mut self, otp_token: O) -> Result<()>
    where
        O: Into<String>,
    {
        let session_req = SessionReq::new(&self.client_code, &self.pin, otp_token).await?;
        let session: SessionRes = session_req.send_data(&self.http).await?;

        self.http.jwt_token(&session.jwt_token);
        self.session = Some(session);
        self.user = Some(self.profile().await?);

        Ok(())
    }

    /// Returns reference to the session already established by call to generate_session
    pub fn session(&self) -> Result<&SessionRes> {
        self.session.as_ref().ok_or_else(|| {
            error!("Session not established");
            Error::SessionEstablishmentError
        })
    }

    /// Returns the current refresh token
    pub fn current_refresh_token(&self) -> Result<&str> {
        self.session().map(|s| s.refresh_token.as_str())
    }

    /// Returns the current feed token
    pub fn current_feed_token(&self) -> Result<&str> {
        self.session().map(|s| s.feed_token.as_str())
    }

    /// Regenerates the authentication tokens using the existing refresh token
    pub async fn token(&self) -> Result<SessionRes> {
        let token_req = TokenReq::new(self.current_refresh_token()?);
        Ok(token_req.send_data(&self.http).await?)
    }

    /// Fetch the complete information of the user who is logged in
    pub async fn profile(&self) -> Result<Profile> {
        let body = HashMap::from([("refreshToken", self.current_refresh_token()?)]);
        Ok(Profile::fetch_data(&self.http, body).await?)
    }

    /// Returns fund, cash and margin information of the user for equity and commodity segments
    pub async fn rms_limit(&self) -> Result<Rms> {
        Ok(Rms::fetch_data(&self.http, &{}).await?)
    }

    /// API session is destroyed by this call and it invalidates the jwt_token
    pub async fn logout(&mut self) -> Result<()> {
        let logout_req = LogoutReq::new(&self.client_code);
        match logout_req.send_data(&self.http).await {
            Ok(()) => (),
            Err(e) => {
                let estr = format!("{e:?}");
                debug!("{estr}");
                if !estr.contains("Logout Successfully") {
                    return Err(Error::UtilsError(e));
                }
            }
        };
        self.session.take();
        self.user.take();
        Ok(())
    }

    /// Returns a new create rule instance to be configured
    pub fn new_create_rule<S, T>(trading_symbol: S, symbol_token: T) -> CreateRuleReq
    where
        S: Into<String>,
        T: Into<String>,
    {
        CreateRuleReq::new(trading_symbol, symbol_token)
    }

    /// Sends the create rule request
    pub async fn create_rule(&self, create_rule_req: &CreateRuleReq) -> Result<CreateRuleRes> {
        Ok(create_rule_req.send_data(&self.http).await?)
    }

    /// Returns a new modify rule instance to be configured
    pub fn new_modify_rule<I, T>(id: I, symbol_token: T) -> ModifyRuleReq
    where
        I: Into<String>,
        T: Into<String>,
    {
        ModifyRuleReq::new(id, symbol_token)
    }

    /// Sends the modify rule request
    pub async fn modify_rule(&self, modify_rule_req: &ModifyRuleReq) -> Result<ModifyRuleRes> {
        Ok(modify_rule_req.send_data(&self.http).await?)
    }

    /// Returns a new cancel rule instance to be configured
    pub fn new_cancel_rule<I, T>(id: I, symbol_token: T, exchange: ExchangeType) -> CancelRuleReq
    where
        I: Into<String>,
        T: Into<String>,
    {
        CancelRuleReq::new(id, symbol_token, exchange)
    }

    /// Sends the cancel rule request
    pub async fn cancel_rule(&self, cancel_rule_req: &CancelRuleReq) -> Result<CancelRuleRes> {
        Ok(cancel_rule_req.send_data(&self.http).await?)
    }

    /// Returns a new detail rule instance to be configured
    pub fn new_rule_detail(id: u64) -> RuleDetailReq {
        RuleDetailReq::new(id)
    }

    /// Sends the detail rule request
    pub async fn rule_detail(&self, rule_detail_req: &RuleDetailReq) -> Result<RuleDetailRes> {
        Ok(rule_detail_req.send_data(&self.http).await?)
    }

    /// Returns a new list rule instance to be configured
    pub fn new_rule_list(status: Vec<RuleType>, page: usize, count: usize) -> RuleListReq {
        RuleListReq::new(status, page, count)
    }

    /// Sends the list rule request
    pub async fn rule_list(&self, rule_list_req: &RuleListReq) -> Result<RuleListRes> {
        Ok(rule_list_req.send_data(&self.http).await?)
    }

    /// Returns a new place order instance to be configured
    pub fn new_place_order<S, T>(
        trading_symbol: S,
        symbol_token: T,
        transaction_type: TransactionType,
    ) -> PlaceOrderReq
    where
        S: Into<String>,
        T: Into<String>,
    {
        PlaceOrderReq::new(trading_symbol, symbol_token, transaction_type)
    }

    /// Places the configured order
    pub async fn place_order(&self, order_req: &PlaceOrderReq) -> Result<PlaceOrderRes> {
        Ok(order_req.send_data(&self.http).await?)
    }

    /// Returns a new modify order instance to be further configured by the caller
    pub fn new_modify_order<S, T, O>(
        trading_symbol: S,
        symbol_token: T,
        order_id: O,
    ) -> ModifyOrderReq
    where
        S: Into<String>,
        T: Into<String>,
        O: Display,
    {
        ModifyOrderReq::new(trading_symbol, symbol_token, order_id)
    }

    /// Modifies the provided order
    pub async fn modify_order(&self, modify_order_req: &ModifyOrderReq) -> Result<ModifyOrderRes> {
        modify_order_req.send(&self.http).await
    }

    /// Returns a new cancel order instance to be further configured by the caller
    pub fn new_cancel_order<O>(order_variety: OrderVariety, order_id: O) -> CancelOrderReq
    where
        O: Display,
    {
        CancelOrderReq::new(order_variety, order_id)
    }

    /// Cancels the provided order
    pub async fn cancel_order(&self, cancel_order_req: &CancelOrderReq) -> Result<CancelOrderRes> {
        cancel_order_req.send(&self.http).await
    }

    /// Fetches the order book
    pub async fn order_book(&self) -> Result<Vec<OrderBook>> {
        Ok(OrderBook::fetch_vec(&self.http, &{}).await?)
    }

    /// Fetches the order status by its id
    pub async fn order_status<O>(&self, unique_order_id: O) -> Result<IndividualOrderStatus>
    where
        O: Into<String>,
    {
        Ok(IndividualOrderStatus::fetch_data(&self.http, unique_order_id).await?)
    }

    /// Fetches the trade book
    pub async fn trade_book(&self) -> Result<Vec<TradeBook>> {
        Ok(TradeBook::fetch_vec(&self.http, &{}).await?)
    }

    /// Returns a new instance for LTP data request
    pub fn new_ltp_data<S, T>(
        exchange: ExchangeType,
        trading_symbol: S,
        trading_token: T,
    ) -> LtpDataReq
    where
        S: Into<String>,
        T: Into<String>,
    {
        LtpDataReq::new(exchange, trading_symbol, trading_token)
    }

    /// Sends the LTP data request
    pub async fn ltp_data(&self, ltp_data_req: &LtpDataReq) -> Result<LtpDataRes> {
        Ok(ltp_data_req.send_data(&self.http).await?)
    }

    /// Returns current portfolio holdings
    pub async fn holdings(&self) -> Result<Vec<Holding>> {
        Ok(Holding::fetch_vec(&self.http, &{}).await?)
    }

    /// Returns all the portfolio holdings
    pub async fn all_holdings(&self) -> Result<AllHoldings> {
        Ok(AllHoldings::fetch_data(&self.http, &{}).await?)
    }

    /// Returns the portfolio position holdings
    pub async fn positions(&self) -> Result<Vec<Position>> {
        Ok(Position::fetch_vec(&self.http, &{}).await?)
    }

    /// Returns a new instance for convert position request
    pub fn new_convert_position<S, T>(trading_symbol: T, quantity: usize) -> ConvertPositionReq
    where
        T: Into<String>,
    {
        ConvertPositionReq::new(trading_symbol, quantity)
    }

    /// Sends the convert position request
    pub async fn convert_position(&self, convert_position_req: &ConvertPositionReq) -> Result<()> {
        Ok(convert_position_req.send_data(&self.http).await?)
    }

    /// Returns a new instance for Market data request
    pub fn new_market_data(
        mode: MarketMode,
        exchange_tokens: HashMap<ExchangeType, Vec<String>>,
    ) -> MarketDataReq
where {
        MarketDataReq::new(mode, exchange_tokens)
    }

    /// Sends the Market data request
    pub async fn market_data(&self, market_data_req: &MarketDataReq) -> Result<MarketDataRes> {
        Ok(market_data_req.send_data(&self.http).await?)
    }

    /// get brokerage calculation
    pub async fn brokerage(&self, brokerage_req: BrokerageReq) -> Result<BrokerageResp> {
        Ok(brokerage_req.send_data(&self.http).await?)
    }

    /// Returns a new instance for Candle data request
    pub fn new_candle_data<S, F, T>(
        exchange: MarketDataExchange,
        symbol_token: S,
        interval: Interval,
        from_date: F,
        to_date: T,
    ) -> Result<CandleDataReq>
    where
        S: Into<String>,
        F: AsRef<str>,
        T: AsRef<str>,
    {
        CandleDataReq::new(exchange, symbol_token, interval, from_date, to_date)
    }

    /// Sends the Candle data request
    pub async fn candle_data(&self, candle_data_req: &CandleDataReq) -> Result<CandleDataRes> {
        Ok(candle_data_req.send_data(&self.http).await?)
    }

    /// Searches the scrip
    pub async fn search_scrip<S>(&self, exchange: ExchangeType, scrip: S) -> Result<SearchScripRes>
    where
        S: Into<String>,
    {
        let req = SearchScripReq::new(exchange, scrip);
        Ok(req.send_data(&self.http).await?)
    }

    /// Nse intraday scrips
    pub async fn nse_intraday_scrips(&self) -> Result<Vec<IntradayScrip>> {
        Ok(match IntradayScrip::fetch_vec(&self.http, &{}).await {
            Ok(scrips) => scrips,
            Err(e) => {
                if let UtilsError::ReqwestError(e) = &e {
                    if let Some(err) = e.source() {
                        let err = err.to_string();
                        let erc = "invalid type: string \"\", expected a sequence";
                        trace!("{err}");
                        if err.contains(erc) {
                            return Ok(vec![]);
                        }
                    }
                }
                return Err(e.into());
            }
        })
    }

    /// Returns a new margin position to calculate the margin
    pub fn new_margin_calculator_position<T, P>(
        exchange: ExchangeType,
        product_type: ProductType,
        trade_type: TransactionType,
        token: T,
        price: P,
        quantity: usize,
    ) -> MarginCalculatorPosition
    where
        T: Into<String>,
        P: Into<f64>,
    {
        MarginCalculatorPosition::new(
            exchange,
            product_type,
            trade_type,
            token,
            price.into(),
            quantity,
        )
    }

    /// Sends the margin calculation request
    pub async fn calculate_margin<P>(&self, positions: P) -> Result<MarginCalculatorRes>
    where
        P: AsRef<[MarginCalculatorPosition]>,
    {
        let mut margin_calc_req = MarginCalculatorReq::new();
        margin_calc_req.add_positions(positions);
        Ok(margin_calc_req.send_data(&self.http).await?)
    }
}
