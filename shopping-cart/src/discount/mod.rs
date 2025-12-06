pub struct DiscountCalculator {
    threshold: f64,
    discount_rate: f64,
}

impl DiscountCalculator {
    pub fn new() -> Self {
        Self {
            threshold: 100.0,    // 满 100 元
            discount_rate: 0.1,  // 打 9 折
        }
    }
    
    pub fn apply_discount(&self, subtotal: f64, item_count: usize) -> f64 {
        // 满减优惠
        let mut total = subtotal;
        
        if subtotal >= self.threshold {
            total = subtotal * (1.0 - self.discount_rate);
        }
        
        // 额外优惠：购买3件及以上商品额外95折
        if item_count >= 3 {
            total *= 0.95;
        }
        
        total
    }
}

impl Default for DiscountCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_no_discount() {
        let calc = DiscountCalculator::new();
        assert_eq!(calc.apply_discount(50.0, 1), 50.0);
    }
    
    #[test]
    fn test_threshold_discount() {
        let calc = DiscountCalculator::new();
        // 满100打9折: 100 * 0.9 = 90
        assert_eq!(calc.apply_discount(100.0, 1), 90.0);
    }
    
    #[test]
    fn test_multiple_items_discount() {
        let calc = DiscountCalculator::new();
        // 满100打9折，3件以上再打95折: 100 * 0.9 * 0.95 = 85.5
        assert_eq!(calc.apply_discount(100.0, 3), 85.5);
    }
}
