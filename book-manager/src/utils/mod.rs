pub mod validators;

pub fn format_price(price: f64) -> String {
    format!("¥{:.2}", price)
}
