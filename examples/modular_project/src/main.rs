// 主入口文件
// 演示如何组织一个模块化的 Rust 项目

// 声明模块
mod config;
mod models;
mod handlers;
mod services;
mod utils;

// 导入需要的类型
use config::Config;
use models::User;
use handlers::user_handler;
use services::auth_service;

fn main() {
    println!("=== 模块化项目示例 ===\n");
    
    // 1. 加载配置
    let config = Config::load();
    println!("配置: {}", config.app_name);
    println!("版本: {}", config.version);
    println!();
    
    // 2. 创建用户
    let user = User::new(1, "Alice", "alice@example.com");
    println!("用户信息:");
    println!("  ID: {}", user.id());
    println!("  姓名: {}", user.name());
    println!("  邮箱: {}", user.email());
    println!();
    
    // 3. 验证用户
    let token = "fake-token-123";
    if auth_service::authenticate(token) {
        println!("✓ 用户认证成功");
    } else {
        println!("✗ 用户认证失败");
    }
    println!();
    
    // 4. 处理用户请求
    match user_handler::get_user(1) {
        Ok(user) => {
            println!("获取用户成功:");
            println!("  {}", user.name());
        }
        Err(e) => {
            println!("获取用户失败: {}", e);
        }
    }
    println!();
    
    // 5. 使用工具函数
    let email = "test@example.com";
    if utils::validation::is_valid_email(email) {
        println!("✓ 邮箱格式正确: {}", email);
    } else {
        println!("✗ 邮箱格式错误: {}", email);
    }
    
    println!("\n✅ 程序运行完成");
}
