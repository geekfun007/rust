// 服务模块根文件
// 包含业务逻辑服务

pub mod auth_service;
pub mod database_service;

// 重导出常用服务
pub use auth_service::authenticate;
pub use database_service::connect;
