// 验证工具函数

pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

pub fn is_valid_username(username: &str) -> bool {
    username.len() >= 3 && username.len() <= 20
}

pub fn is_valid_password(password: &str) -> bool {
    password.len() >= 8
}
