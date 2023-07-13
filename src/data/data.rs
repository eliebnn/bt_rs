use chrono::Duration;
use chrono::naive::NaiveDateTime;

use rand::{Rng,SeedableRng};
use rand_chacha::ChaCha8Rng;

use polars::prelude::*;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub struct DataFactory {
    pub data: DataFrame
}

impl DataFactory {

    pub fn new(size: i32) -> DataFactory {

        let batch_size = size;
        let mut rng = ChaCha8Rng::seed_from_u64(1);
        
        let datetime_str = "2023-06-30 12:30:00";
        let local_now = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").expect("Failed to parse datetime");
    
        // --------

        let prices: Vec<f64> = (0..batch_size).map(|_| rng.gen_range(-15..16) as f64).collect::<Vec<f64>>().iter()
            .scan(100.0, |state, &price| {
                *state *= 1.0 + (price / 10000.0);
                Some(*state)
            }).collect();

        let dates: Vec<_> = (0..batch_size).map(|i| (local_now - Duration::minutes(i as i64)).timestamp()).collect();

        let dates_ser = Series::new("timestamp", &dates);
        let prices_ser = Series::new("close", &prices);
    
        let data: DataFrame = DataFrame::new(vec![dates_ser, prices_ser]).expect("Failed to create DataFrame");

        DataFactory {
            data: data
        }
    }

    pub fn from_csv(path: &str) -> Result<DataFrame, Box<dyn Error>> {

        let file_path = path;

        let file = File::open(file_path).expect("Could not open file");
        let reader = BufReader::new(file);
    
        let df = CsvReader::new(reader)
            .infer_schema(Some(100))  // Use up to 100 records to infer the schema
            .has_header(true)  // Tell the reader that your CSV has a header
            .finish()
            .expect("Could not read CSV");

        Ok(df)
    }
}

// fn main() {

//     let batch_size = 50;
//     let mut rng = ChaCha8Rng::seed_from_u64(1);
    
//     let datetime_str = "2023-06-30 12:30:00";
//     let local_now = NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S").expect("Failed to parse datetime");

//     // --------

//     let prices: Vec<i32> = (0..batch_size).map(|_| rng.gen_range(5..10)).collect();
//     let dates: Vec<_> = (0..batch_size).map(|i| (local_now - Duration::minutes(i)).timestamp()).collect();

//     println!("{:?}", dates);
//     println!("{:?}", prices);

//     let dates_ser = Series::new("dates", &dates);
//     let prices_ser = Series::new("prices", &prices);

//     let df: DataFrame = DataFrame::new(vec![dates_ser, prices_ser]).expect("Failed to create DataFrame");
//     let filter_dt = NaiveDateTime::parse_from_str("2023-06-30 12:00:00", "%Y-%m-%d %H:%M:%S").expect("Failed to parse datetime").timestamp();

//     let filtered_range_df = df.filter(&df.column("dates").expect("foo").gt(filter_dt).unwrap()).unwrap();

//     // let filtered_range_df = df.filter(col("dates").gt_eq(start.timestamp_millis()),)


//     // let conf = RollingOptionsImpl{window_size: Duration::new(2), min_periods: 2, ..RollingOptionsImpl::default()};

//     // let mut foo = df.column("ages").expect("Error foo").rolling_mean(conf.clone()).expect("Error bar");
//     // foo.rename("rolling_avg");

//     // let mut bar = df.column("ages").expect("Error foo").cumsum(false);
//     // bar.rename("cumsum");


//     println!("{:?}", df);
//     println!("{:?}", filtered_range_df);

// }
