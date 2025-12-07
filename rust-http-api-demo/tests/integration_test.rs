/// 集成测试示例
/// 
/// 测试完整的 API 端点

use serde_json::json;

// 注意：这些测试需要运行中的服务器
// 在实际项目中，你可能想要：
// 1. 使用测试数据库
// 2. 在测试前后进行清理
// 3. 使用 testcontainers 等工具

#[tokio::test]
async fn test_health_endpoint() {
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:3000/health")
        .send()
        .await
        .expect("请求失败");
    
    assert_eq!(response.status(), 200);
    
    let body: serde_json::Value = response.json().await.expect("解析 JSON 失败");
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn test_create_user() {
    let client = reqwest::Client::new();
    
    let new_user = json!({
        "username": "testuser",
        "email": "test@example.com",
        "full_name": "Test User"
    });
    
    let response = client
        .post("http://127.0.0.1:3000/api/users")
        .json(&new_user)
        .send()
        .await
        .expect("请求失败");
    
    assert_eq!(response.status(), 201);
    
    let body: serde_json::Value = response.json().await.expect("解析 JSON 失败");
    assert_eq!(body["username"], "testuser");
    assert_eq!(body["email"], "test@example.com");
}

#[tokio::test]
async fn test_validation_error() {
    let client = reqwest::Client::new();
    
    // 无效的用户名（太短）
    let invalid_user = json!({
        "username": "ab",
        "email": "invalid@example.com"
    });
    
    let response = client
        .post("http://127.0.0.1:3000/api/users")
        .json(&invalid_user)
        .send()
        .await
        .expect("请求失败");
    
    assert_eq!(response.status(), 400);
}
