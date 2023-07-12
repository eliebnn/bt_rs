use polars::prelude::*;
use chrono::{Duration, DateTime, NaiveDateTime, Utc};

use super::strategy_config::StrategyConfig;
use super::execution::Side::{Buy, Sell};
use super::execution::{Execution, Status};

use std::error::Error;

#[derive(Debug)]
pub struct StrategyCase {
    pub bar_data: DataFrame,
    quant_data: DataFrame,
    strategy_config: StrategyConfig,
}

impl StrategyCase {
    
    pub fn new(data: &DataFrame, sc: StrategyConfig) -> Result<StrategyCase, Box<dyn Error>> {

        let bar_data: DataFrame = match sc.datetime {
            Some(st_tsp) => {
                
                let ed_dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(st_tsp, 0), Utc);
                let ed_tsp = (ed_dt + Duration::days(sc.max_holding_days + 1)).timestamp();              
                
                let tmp_data = data.clone();
                let tsp_series = tmp_data.column("timestamp")?.i64()?;

                let mask_gt = tsp_series.gt(st_tsp);
                let mask_lt = tsp_series.lt(ed_tsp);
                
                let mask_range = mask_gt & mask_lt;
                
                tmp_data.filter(&mask_range)?.sort(["timestamp"], false)?
            },

                // data.filter(&data.column("timestamp")?.gt(st_tsp)?)?},
            None => {data.clone().sort(["timestamp"], false)?}
        };

        let mut df = bar_data.clone();
        let (rows, cols) = df.shape();
        let fees = sc.fees();

        let price_in_gross = df.head(Some(sc.period_twap_in)).column("close").expect("foo").mean().ok_or("Error on Net Price calculation.")?;
        let net_price: f64 = price_in_gross * (1.0 + fees);

        // ----

        let mut net_mm_price_ser = df.column("close")?.clone().f64()?.apply(|price| price * (1.0 - fees));
        net_mm_price_ser.rename("price_out_mm_net");

        let mut perf_ser = match sc.side {
            Buy => df.column("close")?.clone().f64()?.apply(|price| (price / net_price - 1.0) * 10000.0),
            Sell => df.column("close")?.clone().f64()?.apply(|price| (1.0 - (price / net_price)) * 10000.0)
        };
        perf_ser.rename("perf_bps");


        let sl: Vec<bool> = perf_ser.into_iter().map(|perf| perf.unwrap() <= -(sc.stop_loss_bps as i32) as f64).collect();
        let tp: Vec<bool> = perf_ser.into_iter().map(|perf| perf.unwrap() >= sc.take_profit_bps  as f64).collect();

        let prices_in = Series::new("price_in", (0..rows).map(|_| net_price).collect::<Vec<f64>>());
        let sl = Series::new("stop_loss_hit", sl);
        let tp = Series::new("take_profit_hit", tp);
    
        let mut df = df.with_column(perf_ser)?.with_column(net_mm_price_ser)?.clone();
        let result = df.with_column(prices_in)?.with_column(sl)?.with_column(tp)?.clone();

        println!("\r\nQuant Data: {:?}\r\n", result);

        Ok(StrategyCase {
            bar_data: bar_data,
            quant_data: result,
            strategy_config: sc,
        })
    }

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
            _ => panic!("not a Int compatible value")
        };
    }

    pub fn unwind(&self) -> Result<(i64, f64, Status), Box<dyn Error>> {

        let df = &self.quant_data;
        let twap_out = self.strategy_config.period_twap_out;

        // --

        let tp_mask = df.column("take_profit_hit")?.bool()?.clone();
        let tp_df = df.filter(&tp_mask)?;

        let sl_mask = df.column("stop_loss_hit")?.bool()?.clone();
        let sl_df = df.filter(&sl_mask)?;

        // --

        let mh_dt: i64 = StrategyCase::to_i64(df.tail(Some(twap_out)).head(Some(1)).column("timestamp")?.get(0)?);
        let tp_dt: i64 = if tp_df.shape().0 != 0 {StrategyCase::to_i64(tp_df.column("timestamp")?.get(0)?)} else {mh_dt};
        let sl_dt: i64 = if sl_df.shape().0 != 0 {StrategyCase::to_i64(sl_df.column("timestamp")?.get(0)?)} else {mh_dt};

        println!("{:?}, {:?}, {:?}", mh_dt, tp_dt, sl_dt);

        let binding = [mh_dt, tp_dt, sl_dt];
        let unwind_dt = binding.iter().min().ok_or("Error while computing unwind timestamp.")?;

        let (unwind_price, status) = 
        
        if *unwind_dt == mh_dt {
            (self.max_holding_period(*unwind_dt)?, Status::MaxHolding)
        }

        else if *unwind_dt == tp_dt {
            (self.take_profit(*unwind_dt)?, Status::TakeProfit)
        }

        else if *unwind_dt == sl_dt {
            (self.stop_loss(*unwind_dt)?, Status::StopLoss)
        }
        else {
            panic!("Unwind timestamp is neither unwind_dt, sl_dt nor sl_dt")
        };

        Ok((*unwind_dt, unwind_price, status))
    }



    fn max_holding_period(&self, unwind_dt: i64) -> Result<f64, Box <dyn Error>> {

        let df = &self.quant_data;
        let twap_out = self.strategy_config.period_twap_out;

        let sub_df = df.filter(&df.column("timestamp")?.gt(unwind_dt)?)?;
        let unwind_price: f64 = sub_df.head(Some(twap_out)).column("price_out_mm_net")?.mean().ok_or("Error while getting Unwind price for: Max Holding Period.")?;

        Ok(unwind_price)
    }


    fn take_profit(&self, unwind_dt: i64) -> Result<f64, Box <dyn Error>> {
        // To Do: Should also implement the trailing stop feature

        let df = &self.quant_data;
        let twap_out = self.strategy_config.period_twap_out;

        let sub_df = df.filter(&df.column("timestamp")?.gt(unwind_dt)?)?;
        let unwind_price: f64 = sub_df.head(Some(twap_out)).column("price_out_mm_net")?.mean().ok_or("Error while getting Unwind price for: Max Holding Period.")?;

        Ok(unwind_price)
    }


    fn stop_loss(&self, unwind_dt: i64) -> Result<f64, Box <dyn Error>> {
        // To Do: Should also implement stop loss related features

        let df = &self.quant_data;
        let twap_out = self.strategy_config.period_twap_out;

        let sub_df = df.filter(&df.column("timestamp")?.gt(unwind_dt)?)?;
        let unwind_price: f64 = sub_df.head(Some(twap_out)).column("price_out_mm_net")?.mean().ok_or("Error while getting Unwind price for: Max Holding Period.")?;

        Ok(unwind_price)
    }

    fn get_execution(&self, unwind_dt: i64, unwind_price: f64, status: Status) -> Result<Execution, Box <dyn Error>> {

        let df = &self.quant_data;
        let side = self.strategy_config.side;

        let datetime_int = StrategyCase::to_i64(df.column("timestamp")?.get(0)?);
        let price_in = StrategyCase::to_f64(df.column("price_in")?.get(0)?);

        // let datetime_out = unwind_dt;       
        // let price_out = unwind_price;

        let exec: Execution = Execution::new(price_in, unwind_price, status, datetime_int, unwind_dt, side);

        Ok(exec)

    }

    pub fn run(&self) -> Result<Execution, Box <dyn Error>>{

        let (unwind_dt, unwind_price, status) = match self.unwind() {
            Ok((a, b, c)) => (a, b, c),
            Err(err) => {println!("{}", err); (0, 0.0, Status::StatusError)},
        };

        self.get_execution(unwind_dt, unwind_price, status)
    }
}