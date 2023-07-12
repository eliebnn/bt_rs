
#[derive(Debug)]
pub struct TradeConfig {
    period_twap_in : u32,
    period_twap_out : u32,
    trading_costs_bps : u32,
    take_profit_bps : u32,
    stop_loss_bps : u32,
    take_profit_trailing : bool,
    take_profit_trailing_period : u32,
    take_profit_trailing_tolerance_bps : u32
}

impl TradeConfig {
    pub fn default() -> TradeConfig {
        TradeConfig{
            period_twap_in : 10,
            period_twap_out : 10,
            trading_costs_bps : 0,
            take_profit_bps : 50,
            stop_loss_bps : 50,
            take_profit_trailing : false,
            take_profit_trailing_period : 5,
            take_profit_trailing_tolerance_bps : 5
        }
    }
}