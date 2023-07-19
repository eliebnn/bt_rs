use chrono::Duration;
use chrono::naive::NaiveDateTime;

use rand::{Rng,SeedableRng};
use rand_chacha::ChaCha8Rng;

use polars::prelude::*;

use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, BufRead};

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

    pub fn write_csv(path: &str, df: &mut DataFrame) {

        let file = File::create(path).expect("Could not create file");
        CsvWriter::new(file).has_header(true).finish(df).expect("Could not write DataFrame to CSV");
    }

    pub fn read_file(path: &str) -> Vec<String> {

        let mut lines: Vec<String> = vec![];

        let path = Path::new(path);
        let display = path.display();
    
        let file = match File::open(&path) {
            Err(error) => panic!("couldn't open {}: {}", display, error.to_string()),
            Ok(file) => file,
        };
        
        let reader = BufReader::new(file);
    
        for (index, line) in reader.lines().enumerate() {
            lines.push(line.unwrap());
        }

        lines

    }
}

