use std::error::Error;
use csv::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    date: String,
    asset: String,
    opened_amount: f32,
    market_value: f32,
}



pub fn read_csv_file(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(path)?;
    let headers = reader.headers()?;
    println!("{:?}", headers);

    for result in reader.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

