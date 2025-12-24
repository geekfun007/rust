// 处理器模块根文件

pub mod user_handler;
pub mod post_handler;

// 可以在这里定义通用的处理器逻辑
pub type Result<T> = std::result::Result<T, String>;
