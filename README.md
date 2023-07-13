# bt_rs

```rust
use polars::prelude::*;

mod data;
mod models;

use data::data::DataFactory;

use models::strategy_config::StrategyConfig;
use models::strategy_case::StrategyCase;
use models::strategy::Strategy;
use models::execution::Side;


fn main() {

    // Setting hard coded / programmatic settings

    let max_holding_days: i64 = 7;
    let period_twap_in: usize = 15;
    let period_twap_out: usize = 15;
    let trading_costs_bps: i32 = 10;
    let take_profit_bps: usize = 250;
    let stop_loss_bps: usize = 250;

    let scs: Vec<StrategyConfig> = (0..1000).map(|_| StrategyConfig::new(Side::Buy, StrategyConfig::get_datetime("2023-01-12 12:30:00"), max_holding_days, period_twap_in, period_twap_out, trading_costs_bps, take_profit_bps, stop_loss_bps)).collect();

    let data = DataFactory::new(60*24*365);
    let bar_df = data.data;

    // Setting CSV based settings

    // let scs_df = DataFactory::from_csv("D:/projects/data/btc/trades_signals.csv").unwrap();
    // let bar_df = DataFactory::from_csv("D:/projects/data/btc/btcusd_1min.csv").unwrap();
    // let scs = StrategyConfig::from_csv(&scs_df);

    // Instantiate the backtesting Strategy wrapper

    use std::time::Instant;
    let now = Instant::now();

    let strategy = Strategy::new(bar_df, scs);
    let executions = strategy.run();

    let elapsed = now.elapsed();
    println!("\r\n##################################################################\r\n
    Elapsed: {:.2?} - For {} Executions
    \r\n##################################################################\r\n", elapsed, executions.len());

    println!("Executions: {:#?}", executions);

}


```