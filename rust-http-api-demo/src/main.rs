mod db;
mod handlers;
mod middleware;
mod models;
mod services;

use axum::{
    middleware as axum_middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::db::{create_pool, UserRepository};
use crate::handlers::*;
use crate::middleware::*;
use crate::services::UserService;

#[tokio::main]
async fn main() {
    // 加载环境变量
    dotenv::dotenv().ok();
    
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_http_api_demo=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    tracing::info!("启动应用程序...");
    
    // 创建数据库连接池
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://./app.db".to_string());
    
    tracing::info!("连接数据库: {}", database_url);
    let pool = create_pool(&database_url)
        .await
        .expect("无法创建数据库连接池");
    
    // 运行数据库迁移
    db::pool::run_migrations(&pool)
        .await
        .expect("数据库迁移失败");
    
    // 创建 Repository 和 Service
    let user_repository = UserRepository::new(pool.clone());
    let user_service = UserService::new(user_repository);
    
    // 配置 CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // 构建路由
    let app = Router::new()
        // 健康检查
        .route("/health", get(health_check))
        
        // 用户 API 路由
        .route("/api/users", post(create_user))
        .route("/api/users", get(list_users))
        .route("/api/users/:id", get(get_user))
        .route("/api/users/:id", put(update_user))
        .route("/api/users/:id", delete(delete_user))
        
        // 添加状态
        .with_state(user_service)
        
        // 添加中间件
        .layer(axum_middleware::from_fn(request_id_middleware))
        .layer(axum_middleware::from_fn(logging_middleware))
        .layer(cors);
    
    // 获取服务器地址
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
    
    tracing::info!("服务器运行在 http://{}", addr);
    
    // 创建 listener
    let listener = TcpListener::bind(addr).await.unwrap();
    
    // 启动服务器
    axum::serve(listener, app)
        .await
        .unwrap();
}
