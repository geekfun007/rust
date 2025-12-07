use crate::models::Author;

pub struct AuthorService;

impl AuthorService {
    pub fn new() -> Self {
        Self
    }
    
    pub fn display_author(&self, author: &Author) {
        println!("=== 作者信息 ===");
        println!("ID: {}", author.id);
        println!("姓名: {}", author.name);
    }
}
