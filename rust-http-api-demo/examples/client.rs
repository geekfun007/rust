/// HTTP 客户端示例
/// 
/// 演示如何使用 reqwest 调用 API
/// 
/// 运行方式：
/// 1. 先启动服务器: cargo run
/// 2. 在另一个终端运行: cargo run --example client

use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i64,
    username: String,
    email: String,
    full_name: Option<String>,
}

#[derive(Debug, Serialize)]
struct CreateUserRequest {
    username: String,
    email: String,
    full_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UserListResponse {
    users: Vec<User>,
    total: i64,
    page: i64,
    page_size: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = "http://127.0.0.1:3000";
    let client = reqwest::Client::new();
    
    println!("=== HTTP 客户端示例 ===\n");
    
    // 1. 健康检查
    println!("1. 健康检查");
    let health = client
        .get(format!("{}/health", base_url))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("响应: {}\n", serde_json::to_string_pretty(&health)?);
    
    // 2. 创建用户
    println!("2. 创建用户");
    let new_user = CreateUserRequest {
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        full_name: Some("Alice Smith".to_string()),
    };
    
    let created_user = client
        .post(format!("{}/api/users", base_url))
        .json(&new_user)
        .send()
        .await?
        .json::<User>()
        .await?;
    
    println!("创建的用户: {:?}\n", created_user);
    let user_id = created_user.id;
    
    // 3. 获取用户
    println!("3. 获取用户 ID: {}", user_id);
    let user = client
        .get(format!("{}/api/users/{}", base_url, user_id))
        .send()
        .await?
        .json::<User>()
        .await?;
    
    println!("用户详情: {:?}\n", user);
    
    // 4. 创建更多用户
    println!("4. 创建更多用户");
    for i in 1..=5 {
        let user = CreateUserRequest {
            username: format!("user{}", i),
            email: format!("user{}@example.com", i),
            full_name: Some(format!("User {}", i)),
        };
        
        client
            .post(format!("{}/api/users", base_url))
            .json(&user)
            .send()
            .await?;
        
        println!("  创建用户: user{}", i);
    }
    println!();
    
    // 5. 获取用户列表
    println!("5. 获取用户列表（第1页，每页3条）");
    let list_response = client
        .get(format!("{}/api/users", base_url))
        .query(&[("page", "1"), ("page_size", "3")])
        .send()
        .await?
        .json::<UserListResponse>()
        .await?;
    
    println!("总数: {}", list_response.total);
    println!("当前页: {}", list_response.page);
    println!("每页数量: {}", list_response.page_size);
    println!("用户列表:");
    for user in &list_response.users {
        println!("  - {} ({})", user.username, user.email);
    }
    println!();
    
    // 6. 更新用户
    println!("6. 更新用户 ID: {}", user_id);
    let update_data = json!({
        "username": "alice_updated",
        "full_name": "Alice Updated Smith"
    });
    
    let updated_user = client
        .put(format!("{}/api/users/{}", base_url, user_id))
        .json(&update_data)
        .send()
        .await?
        .json::<User>()
        .await?;
    
    println!("更新后的用户: {:?}\n", updated_user);
    
    // 7. 删除用户
    println!("7. 删除用户 ID: {}", user_id);
    let delete_response = client
        .delete(format!("{}/api/users/{}", base_url, user_id))
        .send()
        .await?;
    
    println!("删除响应状态: {}\n", delete_response.status());
    
    // 8. 验证删除（应该返回 404）
    println!("8. 验证用户已删除");
    let result = client
        .get(format!("{}/api/users/{}", base_url, user_id))
        .send()
        .await?;
    
    println!("状态码: {} (应该是 404)\n", result.status());
    
    // 9. 错误处理示例
    println!("9. 错误处理示例 - 创建无效用户");
    let invalid_user = json!({
        "username": "ab",  // 太短，验证会失败
        "email": "invalid-email",  // 无效邮箱
    });
    
    let error_response = client
        .post(format!("{}/api/users", base_url))
        .json(&invalid_user)
        .send()
        .await?;
    
    println!("状态码: {}", error_response.status());
    if !error_response.status().is_success() {
        let error_body = error_response.json::<serde_json::Value>().await?;
        println!("错误响应: {}\n", serde_json::to_string_pretty(&error_body)?);
    }
    
    // 10. 并发请求示例
    println!("10. 并发请求示例");
    let futures: Vec<_> = (1..=3)
        .map(|i| {
            let client = client.clone();
            let base_url = base_url.to_string();
            async move {
                let user = CreateUserRequest {
                    username: format!("concurrent{}", i),
                    email: format!("concurrent{}@example.com", i),
                    full_name: Some(format!("Concurrent User {}", i)),
                };
                
                client
                    .post(format!("{}/api/users", base_url))
                    .json(&user)
                    .send()
                    .await
            }
        })
        .collect();
    
    let results = futures::future::join_all(futures).await;
    println!("并发创建 {} 个用户", results.len());
    for (i, result) in results.iter().enumerate() {
        match result {
            Ok(response) => println!("  用户 {} 状态: {}", i + 1, response.status()),
            Err(e) => println!("  用户 {} 错误: {}", i + 1, e),
        }
    }
    
    println!("\n=== 示例完成 ===");
    
    Ok(())
}
