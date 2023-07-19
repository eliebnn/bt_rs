use polars::prelude::*;

mod data;
mod models;

use data::data::DataFactory;

use models::strategy_config::StrategyConfig;
use models::strategy_case::StrategyCase;
use models::strategy::Strategy;
use models::execution::Side;


pub struct Case {
    
}

fn main() {

    let scs_df = DataFactory::from_csv("D:/projects/data/btc/signals/trades_signals_1200_1.5_1.5_350_350.csv").unwrap();
    let bar_df = DataFactory::from_csv("D:/projects/data/btc/btcusd_1min.csv").unwrap();
    let scs = StrategyConfig::from_csv(&scs_df);

    // Instantiate the backtesting Strategy wrapper

    use std::time::Instant;
    let now = Instant::now();

    let strategy = Strategy::new(bar_df, scs);
    let executions = strategy.run();

    let elapsed = now.elapsed();
    println!("\r\n##################################################################\r\n
    Elapsed: {:.2?} - For {} Executions
    \r\n##################################################################\r\n", elapsed, executions.len());

    // println!("Executions: {:#?}", executions);

}
