use book_manager::{Book, Author, BookService, AuthorService};

fn main() {
    println!("=== 图书管理系统示例 ===\n");
    
    // 创建作者
    let author = Author::new(1, "鲁迅".to_string());
    
    // 创建图书
    let mut book = Book::new(
        1,
        "狂人日记".to_string(),
        author.id,
        29.99,
    );
    
    // 使用服务显示信息
    let book_service = BookService::new();
    let author_service = AuthorService::new();
    
    author_service.display_author(&author);
    println!();
    book_service.display_book(&book);
    
    // 应用折扣
    println!("\n应用 8 折优惠...");
    book.apply_discount(0.2);
    book_service.display_book(&book);
    
    // 计算多本书的总价
    let books = vec![
        Book::new(2, "呐喊".to_string(), 1, 32.00),
        Book::new(3, "彷徨".to_string(), 1, 28.00),
    ];
    
    let total = book_service.calculate_total(&books);
    println!("\n多本图书总价: ¥{:.2}", total);
}
