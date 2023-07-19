use chrono::naive::NaiveDateTime;

#[derive(Debug, Clone, Copy)]
pub enum Status {
    TakeProfit,
    StopLoss,
    MaxHolding,
    StatusError
}

#[derive(Debug, Clone, Copy)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
pub struct Execution {

    pub price_in : f64,
    pub price_out : f64,
    pub status : Status,
    pub datetime_in : i64,
    pub datetime_out : i64,
    pub side : Side,
}

impl Execution {

    pub fn new(price_in : f64, price_out : f64, status : Status, datetime_in : i64, datetime_out : i64, side : Side) -> Execution {
        Execution{price_in, price_out, status, datetime_in, datetime_out, side}
    }
}