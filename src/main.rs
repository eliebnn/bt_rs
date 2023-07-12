use polars::prelude::*;

mod data;
mod models;

use data::data::DataFactory;

use models::strategy_config::StrategyConfig;
use models::strategy_case::StrategyCase;
use models::strategy::Strategy;
use models::execution::Side;


fn main() {

    // Setting common trade settings

    let max_holding_days: i64 = 7;
    let period_twap_in: usize = 15;
    let period_twap_out: usize = 15;
    let trading_costs_bps: i32 = 10;
    let take_profit_bps: usize = 250;
    let stop_loss_bps: usize = 250;

    // Instantiating the trade settings

    let sc1 = StrategyConfig::new(Side::Buy, StrategyConfig::get_datetime("2023-01-12 12:30:00"), max_holding_days, period_twap_in, period_twap_out, trading_costs_bps, take_profit_bps, stop_loss_bps);
    let sc2 = StrategyConfig::new(Side::Buy, StrategyConfig::get_datetime("2023-02-15 12:30:00"), max_holding_days, period_twap_in, period_twap_out, trading_costs_bps, take_profit_bps, stop_loss_bps);
    let sc3 = StrategyConfig::new(Side::Sell, StrategyConfig::get_datetime("2023-03-25 12:30:00"), max_holding_days, period_twap_in, period_twap_out, trading_costs_bps, take_profit_bps, stop_loss_bps);
    let sc4 = StrategyConfig::new(Side::Buy, StrategyConfig::get_datetime("2023-06-20 12:30:00"), max_holding_days, period_twap_in, period_twap_out, trading_costs_bps, take_profit_bps, stop_loss_bps);

    let scs = vec![sc1, sc2, sc3, sc4];

    // Generating dummy data mimicking tick data

    let data = DataFactory::new(60*24*365);
    let df = data.data;

    // Instantiate the backtesting Strategy wrapper

    let mut strategy = Strategy::new(df, scs);
    strategy.run();

    // Printing resulting trades
    
    println!("{:#?}", strategy.executions);

}
