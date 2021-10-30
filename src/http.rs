use std::error::Error;

pub fn fetch_current_price(coin: &str) -> Result<f32, Box<dyn Error>> {

    //  call API here and return latest value in GBP
    
    let value: f32 = 12344.56;
    Ok(value)
}