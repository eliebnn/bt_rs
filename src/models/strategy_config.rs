use chrono::{DateTime, Utc, NaiveDateTime};
use super::execution::Side;


#[derive(Debug, Clone)]
pub struct StrategyConfig {
    // asset : String,
    pub side : Side,
    pub datetime : Option<i64>,
    pub max_holding_days : i64,
    // notional : usize,

    pub period_twap_in : usize,
    pub period_twap_out : usize,
    trading_costs_bps : i32,
    pub take_profit_bps : usize,
    pub stop_loss_bps : usize,
    // take_profit_trailing : bool,
    // take_profit_trailing_period : usize,
    // take_profit_trailing_tolerance_bps : usize
}

impl StrategyConfig {

    pub fn new(side: Side, datetime: Option<i64>, max_holding_days: i64, period_twap_in : usize, period_twap_out : usize, trading_costs_bps : i32, 
        take_profit_bps : usize,  stop_loss_bps : usize) -> StrategyConfig {

        StrategyConfig{
            side: side,
            datetime: datetime,
            max_holding_days: max_holding_days,
            period_twap_in: period_twap_in,
            period_twap_out: period_twap_out,
            trading_costs_bps: trading_costs_bps,
            take_profit_bps: take_profit_bps,
            stop_loss_bps: stop_loss_bps,
        }
    }

    pub fn default() -> StrategyConfig {
        StrategyConfig{
            // asset : "BTC".to_string(),
            side : Side::Buy,
            datetime : StrategyConfig::get_datetime("2023-06-30 12:00:00"),
            max_holding_days : 7,
            // notional : 1000,

            period_twap_in : 2,
            period_twap_out : 10,
            trading_costs_bps : 10,
            take_profit_bps : 500,
            stop_loss_bps : 500,
            // take_profit_trailing : false,
            // take_profit_trailing_period : 5,
            // take_profit_trailing_tolerance_bps : 5
        }
    }

    pub fn get_datetime(datetime: &str) -> Option<i64> {
        let ndt = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%d %H:%M:%S").expect("Failed to parse datetime");
        Some(DateTime::<Utc>::from_utc(ndt, Utc).timestamp())
    }

    pub fn fees(&self) -> f64 {

        match self.side {
            Side::Buy => {return (self.trading_costs_bps as f64 / 10000.0) as f64;},
            Side::Sell => {return (-self.trading_costs_bps as f64 / 10000.0) as f64;}
        };
    }
}