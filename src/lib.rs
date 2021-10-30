pub fn calculate_profit_loss(
    current_value: f32, 
    amount: f32, 
    purchase_price: f32
) -> f32 {
    let c_value: f32 = current_value * amount;
    let p_value: f32 = purchase_price * amount;
    
    ((c_value - p_value) / p_value) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_profit_loss() {
        let current_value: f32 = 38.0;
        let amount: f32 = 100.0;
        let purchase_price: f32 = 30.0;

        let expected: f32 = 26.666668;

        assert_eq!(expected, calculate_profit_loss(current_value, amount, purchase_price))
    }
}