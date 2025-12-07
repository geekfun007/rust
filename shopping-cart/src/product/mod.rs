#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
}

impl Product {
    pub fn new(id: u32, name: String, price: f64) -> Self {
        assert!(price > 0.0, "价格必须为正数");
        Self { id, name, price }
    }
}
