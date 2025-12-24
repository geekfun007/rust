// 工具模块根文件

pub mod validation;
pub mod formatting;

// 重导出常用工具
pub use validation::is_valid_email;
pub use formatting::format_date;
