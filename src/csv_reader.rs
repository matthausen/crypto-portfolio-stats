use std::error::Error;
use csv::Reader;
use serde::{Serialize, Deserialize};

use crate::lib;
use crate::http;

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    date: String,
    asset: String,
    opened_amount: f32,
    purchase_price: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewRecord {
    date: String,
    asset: String,
    opened_amount: f32,
    purchase_price: f32,
    current_value: f32,
    profit_loss: f32,
}

pub fn read_csv_file(path: &str) -> Result<Vec<NewRecord>, Box<dyn Error>> {
    let mut reader = Reader::from_path(path)?;
    let headers = reader.headers()?;
    println!("{:?}", headers);

    let mut records: Vec<NewRecord> = Vec::new();

    for result in reader.deserialize() {
        let record: Record = result?;
        
        let new_record = NewRecord {
            date: record.date,
            asset: record.asset,
            opened_amount: record.opened_amount,
            purchase_price: record.purchase_price,
            current_value: match http::fetch_current_price(&record.asset){
                Ok(value) => value,
                Err(error) => eprintln!("Could not fetch latest price for coin{}", error)
            },
            profit_loss: lib::calculate_profit_loss(60000.0, record.opened_amount, record.purchase_price),
        };
        
        records.push(new_record)
    }

    println!("Records: {:?}", records);

    Ok(records)
}