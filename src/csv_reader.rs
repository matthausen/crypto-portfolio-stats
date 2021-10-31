use std::error::Error;
use csv::Reader;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
    
    let mut all_assets_values: HashMap<String, f32> = HashMap::new();

    for result in reader.deserialize() {
        let record: Record = result?;

        let name = &record.asset;

        if !all_assets_values.contains_key(name) {
            let value = match http::fetch_current_price(name){
                Ok(value) => value,
                Err(_error) => 0.0
            };
            all_assets_values.insert(
                name.to_string(), value
            );
        }
        
        let new_record = NewRecord {
            date: record.date,
            asset: name.to_string(),
            opened_amount: record.opened_amount,
            purchase_price: record.purchase_price,
            current_value: match all_assets_values.get(name) {
                Some(&v) => v,
                None => 0.0,
            },
            profit_loss: lib::calculate_profit_loss(
                match all_assets_values.get(name) {
                    Some(&v) => v,
                    None => 0.0,
                }, 
                record.opened_amount, 
                record.purchase_price
            ),
        };
        
        records.push(new_record)
    }

    println!("Records: {:?}", records);

    Ok(records)
}