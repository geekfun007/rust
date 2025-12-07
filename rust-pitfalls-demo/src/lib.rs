pub mod ownership;
pub mod borrowing;
pub mod lifetime;
pub mod error_handling;
pub mod performance;
pub mod concurrency;

// 重新导出常用类型
pub use ownership::{Config, LogProcessor};
pub use borrowing::{Cache, Database};
pub use error_handling::{AppError, AppResult};
