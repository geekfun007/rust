// 文章处理器

use crate::models::Post;
use super::Result;

pub fn get_post(id: u32) -> Result<Post> {
    if id > 0 {
        Ok(Post::new(
            id,
            "Sample Post".to_string(),
            "Post content".to_string(),
            1,
        ))
    } else {
        Err("Invalid post ID".to_string())
    }
}

pub fn create_post(title: String, content: String, author_id: u32) -> Result<Post> {
    let id = 1; // 模拟生成的 ID
    Ok(Post::new(id, title, content, author_id))
}
