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

    price_in : f64,
    price_out : f64,
    status : Status,
    datetime_int : i64,
    datetime_out : i64,
    side : Side,
}

impl Execution {

    pub fn new(price_in : f64, price_out : f64, status : Status, datetime_int : i64, datetime_out : i64, side : Side) -> Execution {
        Execution{price_in, price_out, status, datetime_int, datetime_out, side}
    }
}