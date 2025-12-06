use crate::product::Product;
use crate::discount::DiscountCalculator;
use super::item::CartItem;

pub struct Cart {
    items: Vec<CartItem>,
    discount_calculator: DiscountCalculator,
}

impl Cart {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            discount_calculator: DiscountCalculator::new(),
        }
    }
    
    pub fn add_item(&mut self, product: Product, quantity: u32) {
        if let Some(item) = self.items.iter_mut().find(|i| i.product.id == product.id) {
            item.quantity += quantity;
        } else {
            self.items.push(CartItem::new(product, quantity));
        }
    }
    
    pub fn remove_item(&mut self, product_id: u32) {
        self.items.retain(|item| item.product.id != product_id);
    }
    
    pub fn update_quantity(&mut self, product_id: u32, quantity: u32) {
        if let Some(item) = self.items.iter_mut().find(|i| i.product.id == product_id) {
            item.quantity = quantity;
        }
    }
    
    pub fn subtotal(&self) -> f64 {
        self.items.iter().map(|item| item.total()).sum()
    }
    
    pub fn total(&self) -> f64 {
        let subtotal = self.subtotal();
        self.discount_calculator.apply_discount(subtotal, self.items.len())
    }
    
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    
    pub fn display(&self) {
        println!("\n=== 购物车 ===");
        
        if self.items.is_empty() {
            println!("购物车是空的");
            return;
        }
        
        for item in &self.items {
            println!(
                "{} x {} = ¥{:.2}",
                item.product.name,
                item.quantity,
                item.total()
            );
        }
        
        println!("─────────────────────");
        println!("小计: ¥{:.2}", self.subtotal());
        
        let discount = self.subtotal() - self.total();
        if discount > 0.0 {
            println!("优惠: -¥{:.2}", discount);
        }
        
        println!("总计: ¥{:.2}", self.total());
    }
}

impl Default for Cart {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::Product;
    
    #[test]
    fn test_add_item() {
        let mut cart = Cart::new();
        let product = Product::new(1, "商品A".to_string(), 10.0);
        
        cart.add_item(product, 2);
        assert_eq!(cart.item_count(), 1);
        assert_eq!(cart.subtotal(), 20.0);
    }
    
    #[test]
    fn test_remove_item() {
        let mut cart = Cart::new();
        let product = Product::new(1, "商品A".to_string(), 10.0);
        
        cart.add_item(product, 2);
        cart.remove_item(1);
        
        assert_eq!(cart.item_count(), 0);
    }
}
