pub fn validate_price(price: f64) -> bool {
    price > 0.0
}

pub fn validate_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 100
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_price() {
        assert!(validate_price(10.0));
        assert!(!validate_price(0.0));
        assert!(!validate_price(-1.0));
    }
}
