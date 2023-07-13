use chrono::{DateTime, Utc, NaiveDateTime};
use polars::prelude::*;

use super::execution::Side;

fn to_f64(v: AnyValue) -> f64 {
    match v {
        AnyValue::Float64(b) => {return b.into();},
        AnyValue::Float32(b) => {return b.into();},
        _ => panic!("not a float compatible value")
    };
}


fn to_i64(v: AnyValue) -> i64 {
    match v {
        AnyValue::Int64(b) => {return b.into();},
        AnyValue::Int32(b) => {return b.into();},
        AnyValue::Float64(b) => {return b as i64;},
        AnyValue::Float32(b) => {return b as i64;},
        _ => panic!("not a Int compatible value")
    };
}

#[derive(Debug, Clone)]
pub struct StrategyConfig {
    pub side : Side,
    pub datetime : Option<i64>,
    pub max_holding_days : i64,
    pub period_twap_in : usize,
    pub period_twap_out : usize,
    trading_costs_bps : i32,
    pub take_profit_bps : usize,
    pub stop_loss_bps : usize,
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
            side : Side::Buy,
            datetime : StrategyConfig::get_datetime("2023-06-30 12:00:00"),
            max_holding_days : 7,
            period_twap_in : 2,
            period_twap_out : 10,
            trading_costs_bps : 10,
            take_profit_bps : 500,
            stop_loss_bps : 500,
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

    pub fn from_csv(df: &DataFrame) -> Vec<StrategyConfig> {

        let mut ls: Vec<StrategyConfig> = vec![];

        for i in 0..df.height() {

            let row = df.get_row(i).unwrap();

            let datetime: i64 = to_i64(row.0[0].clone());
            let side: Side = if to_i64(row.0[1].clone()) == 1 {Side::Buy} else {Side::Sell};
            let max_holding_days: i64 = to_i64(row.0[2].clone());
            let period_twap_in: usize = to_i64(row.0[3].clone()) as usize;
            let period_twap_out: usize = to_i64(row.0[4].clone()) as usize;
            let trading_costs_bps: i32 = to_i64(row.0[5].clone()) as i32;
            let take_profit_bps: usize = to_i64(row.0[6].clone()) as usize;
            let stop_loss_bps: usize = to_i64(row.0[7].clone()) as usize;        

            let tmp = StrategyConfig::new(side, Some(datetime), max_holding_days, period_twap_in, period_twap_out, trading_costs_bps, take_profit_bps, stop_loss_bps);
            ls.push(tmp);
        }

        ls
    }
}