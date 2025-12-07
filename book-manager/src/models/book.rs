use crate::utils::validators;

#[derive(Debug, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author_id: u32,
    pub price: f64,
}

impl Book {
    pub fn new(id: u32, title: String, author_id: u32, price: f64) -> Self {
        assert!(validators::validate_price(price), "价格必须为正数");
        
        Self {
            id,
            title,
            author_id,
            price,
        }
    }
    
    pub fn set_price(&mut self, new_price: f64) {
        if validators::validate_price(new_price) {
            self.price = new_price;
        } else {
            panic!("无效的价格");
        }
    }
    
    pub fn apply_discount(&mut self, discount: f64) {
        assert!(discount >= 0.0 && discount <= 1.0, "折扣必须在 0-1 之间");
        self.price = self.price * (1.0 - discount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_book() {
        let book = Book::new(1, "测试书籍".to_string(), 1, 29.99);
        assert_eq!(book.id, 1);
        assert_eq!(book.title, "测试书籍");
    }
    
    #[test]
    fn test_apply_discount() {
        let mut book = Book::new(1, "测试".to_string(), 1, 100.0);
        book.apply_discount(0.2);  // 8折
        assert_eq!(book.price, 80.0);
    }
}
