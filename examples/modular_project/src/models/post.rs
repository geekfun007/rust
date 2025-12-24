// 文章模型

pub struct Post {
    id: u32,
    title: String,
    content: String,
    author_id: u32,
}

impl Post {
    pub fn new(id: u32, title: String, content: String, author_id: u32) -> Self {
        Post {
            id,
            title,
            content,
            author_id,
        }
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
    
    pub fn title(&self) -> &str {
        &self.title
    }
    
    pub fn author_id(&self) -> u32 {
        self.author_id
    }
}
