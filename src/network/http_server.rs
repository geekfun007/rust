// HTTP 服务器

/// # Axum Web 框架基础
pub fn axum_basics_demo() {
    println!("\n=== Axum Web 框架基础 ===");
    
    println!("基本服务器:");
    println!("  use axum::{{routing::get, Router}};");
    println!("  ");
    println!("  #[tokio::main]");
    println!("  async fn main() {{");
    println!("      let app = Router::new()");
    println!("          .route(\"/\", get(handler));");
    println!("      ");
    println!("      axum::Server::bind(&\"0.0.0.0:3000\".parse().unwrap())");
    println!("          .serve(app.into_make_service())");
    println!("          .await");
    println!("          .unwrap();");
    println!("  }}");
    println!("  ");
    println!("  async fn handler() -> &'static str {{");
    println!("      \"Hello, World!\"");
    println!("  }}");
}

/// # 路由
pub fn routing_demo() {
    println!("\n=== 路由 ===");
    
    println!("定义路由:");
    println!("  let app = Router::new()");
    println!("      .route(\"/\", get(index))");
    println!("      .route(\"/users\", get(list_users).post(create_user))");
    println!("      .route(\"/users/:id\", get(get_user).put(update_user).delete(delete_user));");
    
    println!("\n路径参数:");
    println!("  async fn get_user(Path(id): Path<u32>) -> String {{");
    println!("      format!(\"User ID: {{}}\", id)");
    println!("  }}");
    
    println!("\n查询参数:");
    println!("  use axum::extract::Query;");
    println!("  ");
    println!("  #[derive(Deserialize)]");
    println!("  struct Params {{");
    println!("      page: Option<u32>,");
    println!("  }}");
    println!("  ");
    println!("  async fn list(Query(params): Query<Params>) -> String {{");
    println!("      format!(\"Page: {{}}\", params.page.unwrap_or(1))");
    println!("  }}");
}

/// # 请求处理
pub fn request_handling_demo() {
    println!("\n=== 请求处理 ===");
    
    println!("JSON 请求:");
    println!("  use axum::Json;");
    println!("  use serde::{{Deserialize, Serialize}};");
    println!("  ");
    println!("  #[derive(Deserialize)]");
    println!("  struct CreateUser {{");
    println!("      name: String,");
    println!("  }}");
    println!("  ");
    println!("  async fn create_user(Json(user): Json<CreateUser>) -> StatusCode {{");
    println!("      // 处理用户创建");
    println!("      StatusCode::CREATED");
    println!("  }}");
    
    println!("\nJSON 响应:");
    println!("  #[derive(Serialize)]");
    println!("  struct User {{");
    println!("      id: u32,");
    println!("      name: String,");
    println!("  }}");
    println!("  ");
    println!("  async fn get_user() -> Json<User> {{");
    println!("      Json(User {{");
    println!("          id: 1,");
    println!("          name: \"Alice\".to_string(),");
    println!("      }})");
    println!("  }}");
}

/// # 中间件
pub fn middleware_demo() {
    println!("\n=== 中间件 ===");
    
    println!("日志中间件:");
    println!("  use tower_http::trace::TraceLayer;");
    println!("  ");
    println!("  let app = Router::new()");
    println!("      .route(\"/\", get(handler))");
    println!("      .layer(TraceLayer::new_for_http());");
    
    println!("\nCORS 中间件:");
    println!("  use tower_http::cors::CorsLayer;");
    println!("  ");
    println!("  let app = Router::new()");
    println!("      .route(\"/api/users\", get(list_users))");
    println!("      .layer(CorsLayer::permissive());");
    
    println!("\n自定义中间件:");
    println!("  use axum::middleware;");
    println!("  ");
    println!("  async fn auth_middleware(");
    println!("      req: Request<Body>,");
    println!("      next: Next<Body>,");
    println!("  ) -> Result<Response, StatusCode> {{");
    println!("      // 验证逻辑");
    println!("      Ok(next.run(req).await)");
    println!("  }}");
}

/// # 状态共享
pub fn state_sharing_demo() {
    println!("\n=== 状态共享 ===");
    
    println!("使用 State:");
    println!("  use std::sync::{{Arc, Mutex}};");
    println!("  ");
    println!("  struct AppState {{");
    println!("      counter: Mutex<u32>,");
    println!("  }}");
    println!("  ");
    println!("  let shared_state = Arc::new(AppState {{");
    println!("      counter: Mutex::new(0),");
    println!("  }});");
    println!("  ");
    println!("  let app = Router::new()");
    println!("      .route(\"/\", get(handler))");
    println!("      .with_state(shared_state);");
    println!("  ");
    println!("  async fn handler(State(state): State<Arc<AppState>>) -> String {{");
    println!("      let mut counter = state.counter.lock().unwrap();");
    println!("      *counter += 1;");
    println!("      format!(\"Count: {{}}\", counter)");
    println!("  }}");
}

/// # 错误处理
pub fn error_handling_demo() {
    println!("\n=== 错误处理 ===");
    
    println!("返回错误:");
    println!("  use axum::http::StatusCode;");
    println!("  ");
    println!("  async fn handler() -> Result<String, StatusCode> {{");
    println!("      if some_condition {{");
    println!("          Ok(\"success\".to_string())");
    println!("      }} else {{");
    println!("          Err(StatusCode::BAD_REQUEST)");
    println!("      }}");
    println!("  }}");
    
    println!("\n自定义错误响应:");
    println!("  use axum::response::{{IntoResponse, Response}};");
    println!("  ");
    println!("  struct AppError(anyhow::Error);");
    println!("  ");
    println!("  impl IntoResponse for AppError {{");
    println!("      fn into_response(self) -> Response {{");
    println!("          (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string())");
    println!("              .into_response()");
    println!("      }}");
    println!("  }}");
}

/// # 文件上传
pub fn file_upload_demo() {
    println!("\n=== 文件上传 ===");
    
    println!("处理文件上传:");
    println!("  use axum::extract::Multipart;");
    println!("  ");
    println!("  async fn upload(mut multipart: Multipart) -> Result<String, StatusCode> {{");
    println!("      while let Some(field) = multipart.next_field().await.unwrap() {{");
    println!("          let name = field.name().unwrap().to_string();");
    println!("          let data = field.bytes().await.unwrap();");
    println!("          ");
    println!("          // 保存文件");
    println!("          tokio::fs::write(&name, &data).await.unwrap();");
    println!("      }}");
    println!("      Ok(\"Uploaded\".to_string())");
    println!("  }}");
}

/// # WebSocket
pub fn websocket_demo() {
    println!("\n=== WebSocket ===");
    
    println!("WebSocket 服务器:");
    println!("  use axum::{{");
    println!("      extract::ws::{{Message, WebSocket, WebSocketUpgrade}},");
    println!("      response::Response,");
    println!("  }};");
    println!("  ");
    println!("  async fn ws_handler(ws: WebSocketUpgrade) -> Response {{");
    println!("      ws.on_upgrade(handle_socket)");
    println!("  }}");
    println!("  ");
    println!("  async fn handle_socket(mut socket: WebSocket) {{");
    println!("      while let Some(msg) = socket.recv().await {{");
    println!("          if let Ok(msg) = msg {{");
    println!("              if socket.send(msg).await.is_err() {{");
    println!("                  break;");
    println!("              }}");
    println!("          }}");
    println!("      }}");
    println!("  }}");
}

/// # 实战示例：REST API
pub fn rest_api_demo() {
    println!("\n=== 实战示例：REST API ===");
    
    println!("完整的 REST API:");
    println!("  ");
    println!("  #[derive(Serialize, Deserialize)]");
    println!("  struct Todo {{");
    println!("      id: u32,");
    println!("      title: String,");
    println!("      completed: bool,");
    println!("  }}");
    println!("  ");
    println!("  let app = Router::new()");
    println!("      .route(\"/todos\", get(list_todos).post(create_todo))");
    println!("      .route(\"/todos/:id\",");
    println!("          get(get_todo)");
    println!("              .put(update_todo)");
    println!("              .delete(delete_todo)");
    println!("      );");
    
    println!("\n处理函数:");
    println!("  async fn list_todos() -> Json<Vec<Todo>> {{ ... }}");
    println!("  async fn create_todo(Json(todo): Json<Todo>) -> (StatusCode, Json<Todo>) {{ ... }}");
    println!("  async fn get_todo(Path(id): Path<u32>) -> Result<Json<Todo>, StatusCode> {{ ... }}");
    println!("  async fn update_todo(Path(id): Path<u32>, Json(todo): Json<Todo>) -> StatusCode {{ ... }}");
    println!("  async fn delete_todo(Path(id): Path<u32>) -> StatusCode {{ ... }}");
}

/// # HTTP 服务器最佳实践
pub fn http_server_best_practices_demo() {
    println!("\n=== HTTP 服务器最佳实践 ===");
    
    println!("1. 选择框架:");
    println!("   - Axum: 现代、类型安全");
    println!("   - Actix-web: 高性能");
    println!("   - Rocket: 易用性");
    println!("   - Warp: 函数式风格");
    
    println!("\n2. API 设计:");
    println!("   - RESTful 原则");
    println!("   - 清晰的路由");
    println!("   - 版本控制");
    println!("   - 文档化");
    
    println!("\n3. 安全性:");
    println!("   - HTTPS");
    println!("   - CORS 配置");
    println!("   - 身份验证");
    println!("   - 输入验证");
    println!("   - 限流");
    
    println!("\n4. 性能:");
    println!("   - 连接池");
    println!("   - 缓存");
    println!("   - 压缩");
    println!("   - 异步处理");
    
    println!("\n5. 可观测性:");
    println!("   - 日志");
    println!("   - 指标");
    println!("   - 追踪");
    println!("   - 健康检查");
}

/// 运行所有 HTTP 服务器示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║       Rust HTTP 服务器详解         ║");
    println!("╚════════════════════════════════════╝");
    
    axum_basics_demo();
    routing_demo();
    request_handling_demo();
    middleware_demo();
    state_sharing_demo();
    error_handling_demo();
    file_upload_demo();
    websocket_demo();
    rest_api_demo();
    http_server_best_practices_demo();
}
