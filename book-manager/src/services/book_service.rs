use crate::models::Book;

pub struct BookService;

impl BookService {
    pub fn new() -> Self {
        Self
    }
    
    pub fn display_book(&self, book: &Book) {
        println!("=== 图书信息 ===");
        println!("ID: {}", book.id);
        println!("标题: {}", book.title);
        println!("作者ID: {}", book.author_id);
        println!("价格: ¥{:.2}", book.price);
    }
    
    pub fn calculate_total(&self, books: &[Book]) -> f64 {
        books.iter().map(|b| b.price).sum()
    }
}

impl Default for BookService {
    fn default() -> Self {
        Self::new()
    }
}
