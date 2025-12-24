// 认证服务
// 处理用户认证相关逻辑

pub fn authenticate(token: &str) -> bool {
    // 模拟 token 验证
    !token.is_empty() && token.starts_with("fake-token")
}

pub fn generate_token(user_id: u32) -> String {
    format!("token-{}", user_id)
}

pub fn refresh_token(old_token: &str) -> Option<String> {
    if authenticate(old_token) {
        Some("new-token".to_string())
    } else {
        None
    }
}
