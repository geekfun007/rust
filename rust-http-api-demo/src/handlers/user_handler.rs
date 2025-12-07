use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::models::{AppResult, CreateUserRequest, UpdateUserRequest, User};
use crate::services::UserService;

/// 分页查询参数
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: i64,
    
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    10
}

/// 用户列表响应
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<User>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

/// 创建用户处理器
pub async fn create_user(
    State(service): State<UserService>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<(StatusCode, Json<User>)> {
    let user = service.create_user(req).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

/// 获取用户处理器
pub async fn get_user(
    State(service): State<UserService>,
    Path(id): Path<i64>,
) -> AppResult<Json<User>> {
    let user = service.get_user(id).await?;
    Ok(Json(user))
}

/// 获取用户列表处理器
pub async fn list_users(
    State(service): State<UserService>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<UserListResponse>> {
    let users = service.list_users(params.page, params.page_size).await?;
    let total = service.count_users().await?;
    
    Ok(Json(UserListResponse {
        users,
        total,
        page: params.page,
        page_size: params.page_size,
    }))
}

/// 更新用户处理器
pub async fn update_user(
    State(service): State<UserService>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<Json<User>> {
    let user = service.update_user(id, req).await?;
    Ok(Json(user))
}

/// 删除用户处理器
pub async fn delete_user(
    State(service): State<UserService>,
    Path(id): Path<i64>,
) -> AppResult<(StatusCode, Json<Value>)> {
    service.delete_user(id).await?;
    Ok((
        StatusCode::OK,
        Json(json!({ "message": "用户删除成功" })),
    ))
}
