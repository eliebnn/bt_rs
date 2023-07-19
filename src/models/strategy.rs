use std::error::Error;
use polars::prelude::*;
use super::strategy_case::StrategyCase;
use super::strategy_config::StrategyConfig;

use super::execution::{Execution, Status, Side};


#[derive(Debug)]
pub struct Strategy<'a>{
    bar_data: &'a DataFrame,
    strategy_configs: Vec<StrategyConfig>,
    // pub executions: Vec<Execution>,
}

impl Strategy<'_> {

    pub fn new(bar_data: &DataFrame, strategy_configs: Vec<StrategyConfig>) -> Strategy {

        // let mut executions: Vec<Execution> = vec![];
        // Strategy{bar_data: bar_data, strategy_configs: strategy_configs, executions: executions}
        
        Strategy{bar_data: &bar_data, strategy_configs: strategy_configs}
    }

    pub fn run(&self) -> Vec<Execution> {

        let mut executions: Vec<Execution> = vec![];

        for strategy_config in self.strategy_configs.iter() {
            let strategy_case = StrategyCase::new(self.bar_data, strategy_config.clone());

            match strategy_case {
                Ok(case) => {
                    match case.run() {
                        Ok(e) => executions.push(e),
                        Err(err_exec) => println!("Error for executions"),
                    }
                },
                // Err(err_strat) => println!("Error for strategy_config\r\n{:?}\r\n: {:#?}", err_strat, strategy_config),
                // Err(err_strat) => println!("Error for strategy_config\r\n{:?}", err_strat),
                Err(err_strat) => (),
            };
        }

        executions
    }

    pub fn concat(executions: &Vec<Execution>) -> Result<DataFrame, Box<dyn Error>> {

        let mut price_in : Vec<f64> = vec![];
        let mut price_out : Vec<f64> = vec![];
        let mut status : Vec<String> = vec![];
        let mut datetime_in : Vec<i64> = vec![];
        let mut datetime_out : Vec<i64> = vec![];
        let mut side : Vec<String> = vec![];

        for exec in executions.iter(){

            let st: String = match exec.status {
                Status::TakeProfit => "TakeProfit".to_string(),
                Status::StopLoss => "StopLoss".to_string(),
                Status::MaxHolding => "MaxHolding".to_string(),
                Status::StatusError => "StatusError".to_string(),
            };
    
            let si: String = match exec.side {
                Side::Buy => "Buy".to_string(),
                Side::Sell => "Sell".to_string(),
            };
    
            price_in.push(exec.price_in);
            price_out.push(exec.price_out);
            status.push(st);
            datetime_in.push(exec.datetime_in);
            datetime_out.push(exec.datetime_out);
            side.push(si);
        }
    
        let price_in = Series::new("price_in", &price_in);
        let price_out = Series::new("price_out", &price_out);
        let status = Series::new("status", &status);
        let datetime_in = Series::new("datetime_in", &datetime_in);
        let datetime_out = Series::new("datetime_out", &datetime_out);
        let side = Series::new("side", &side);
    
        let mut df: polars::prelude::DataFrame = DataFrame::new(vec![price_in, price_out, status, datetime_in, datetime_out, side]).expect("Failed to create DataFrame");

        Ok(df)
    }
}