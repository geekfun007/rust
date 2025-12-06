use crate::product::Product;

pub struct CartItem {
    pub product: Product,
    pub quantity: u32,
}

impl CartItem {
    pub fn new(product: Product, quantity: u32) -> Self {
        Self { product, quantity }
    }
    
    pub fn total(&self) -> f64 {
        self.product.price * self.quantity as f64
    }
}
