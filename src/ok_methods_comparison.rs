// .ok() vs .ok_or() vs .ok_or_else() 深度对比
//
// 本教程专注于这三个常用的类型转换方法

fn main() {
    println!("=== .ok() vs .ok_or() vs .ok_or_else() 详解 ===\n");
    
    // 1. 概览和类型签名
    demo_overview();
    
    // 2. .ok() - Result → Option
    demo_ok();
    
    // 3. .ok_or() - Option → Result
    demo_ok_or();
    
    // 4. .ok_or_else() - Option → Result (惰性)
    demo_ok_or_else();
    
    // 5. 三者对比
    demo_comparison();
    
    // 6. 性能考虑
    demo_performance();
    
    // 7. 实战场景
    demo_real_world_scenarios();
    
    // 8. 常见模式和最佳实践
    demo_best_practices();
}

// ============================================
// 1. 概览和类型签名
// ============================================
fn demo_overview() {
    println!("--- 1. 概览和类型签名 ---\n");
    
    println!("┌────────────────┬─────────────────┬─────────────────┐");
    println!("│  方法           │  输入类型        │  输出类型        │");
    println!("├────────────────┼─────────────────┼─────────────────┤");
    println!("│  .ok()         │  Result<T, E>   │  Option<T>      │");
    println!("│  .ok_or(err)   │  Option<T>      │  Result<T, E>   │");
    println!("│  .ok_or_else() │  Option<T>      │  Result<T, E>   │");
    println!("└────────────────┴─────────────────┴─────────────────┘");
    println!();
    
    println!("类型签名:");
    println!();
    println!("1. .ok()");
    println!("   impl<T, E> Result<T, E> {{");
    println!("       fn ok(self) -> Option<T>");
    println!("   }}");
    println!();
    
    println!("2. .ok_or()");
    println!("   impl<T> Option<T> {{");
    println!("       fn ok_or<E>(self, err: E) -> Result<T, E>");
    println!("   }}");
    println!();
    
    println!("3. .ok_or_else()");
    println!("   impl<T> Option<T> {{");
    println!("       fn ok_or_else<E, F>(self, err: F) -> Result<T, E>");
    println!("       where F: FnOnce() -> E");
    println!("   }}");
    println!();
    
    println!("核心区别:");
    println!("  📌 .ok()        : 丢弃错误信息");
    println!("  📌 .ok_or()     : 立即求值错误");
    println!("  📌 .ok_or_else(): 惰性求值错误（仅在需要时）");
    println!();
}

// ============================================
// 2. .ok() 详解
// ============================================
fn demo_ok() {
    println!("--- 2. .ok() - Result → Option ---\n");
    
    println!("作用: 将 Result<T, E> 转换为 Option<T>");
    println!("  - Ok(v)  → Some(v)");
    println!("  - Err(_) → None");
    println!("  - 丢弃错误信息，只关心是否成功\n");
    
    // 基础示例
    println!("基础示例:");
    let success: Result<i32, &str> = Ok(42);
    let failure: Result<i32, &str> = Err("出错了");
    
    println!("  Ok(42).ok()      = {:?}", success.ok());
    println!("  Err('出错了').ok() = {:?}", failure.ok());
    println!();
    
    // 实际应用：解析
    println!("实际应用 1: 解析字符串");
    let numbers = vec!["1", "2", "abc", "4", "xyz", "5"];
    let parsed: Vec<i32> = numbers
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    
    println!("  输入: {:?}", numbers);
    println!("  解析成功的: {:?}", parsed);
    println!("  说明: .ok() 配合 filter_map 过滤掉解析失败的");
    println!();
    
    // 实际应用：可选配置
    println!("实际应用 2: 读取可选配置");
    use std::env;
    
    let timeout = env::var("TIMEOUT")
        .ok()  // Result<String, VarError> → Option<String>
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(30);
    
    println!("  TIMEOUT 环境变量: {:?}", env::var("TIMEOUT").ok());
    println!("  使用的超时值: {} 秒", timeout);
    println!("  说明: 配置不存在或无效时使用默认值");
    println!();
    
    // 何时使用 .ok()
    println!("何时使用 .ok():");
    println!("  ✓ 不关心错误的具体内容");
    println!("  ✓ 只需要知道是否成功");
    println!("  ✓ 配合 filter_map 过滤");
    println!("  ✓ 链式调用中间步骤");
    println!();
}

// ============================================
// 3. .ok_or() 详解
// ============================================
fn demo_ok_or() {
    println!("--- 3. .ok_or() - Option → Result ---\n");
    
    println!("作用: 将 Option<T> 转换为 Result<T, E>");
    println!("  - Some(v) → Ok(v)");
    println!("  - None    → Err(提供的错误值)");
    println!("  - 必须提供错误值\n");
    
    // 基础示例
    println!("基础示例:");
    let some_value: Option<i32> = Some(100);
    let none_value: Option<i32> = None;
    
    println!("  Some(100).ok_or('错误') = {:?}", some_value.ok_or("错误"));
    println!("  None.ok_or('错误')      = {:?}", none_value.ok_or("错误"));
    println!();
    
    // 实际应用：必需的配置
    println!("实际应用 1: 读取必需的配置");
    use std::collections::HashMap;
    
    let config: HashMap<&str, &str> = [
        ("host", "localhost"),
        ("port", "8080"),
    ].iter().cloned().collect();
    
    fn get_config<'a>(config: &'a HashMap<&str, &str>, key: &str) -> Result<&'a str, String> {
        config
            .get(key)
            .copied()
            .ok_or(format!("缺少必需的配置项: {}", key))
    }
    
    match get_config(&config, "host") {
        Ok(value) => println!("  host = {}", value),
        Err(e) => println!("  错误: {}", e),
    }
    
    match get_config(&config, "database") {
        Ok(value) => println!("  database = {}", value),
        Err(e) => println!("  错误: {}", e),
    }
    println!();
    
    // 实际应用：查找
    println!("实际应用 2: 查找用户");
    
    fn find_user(id: u32) -> Option<String> {
        match id {
            1 => Some("Alice".to_string()),
            2 => Some("Bob".to_string()),
            _ => None,
        }
    }
    
    let user = find_user(1)
        .ok_or("用户不存在");
    println!("  find_user(1) = {:?}", user);
    
    let user = find_user(999)
        .ok_or("用户不存在");
    println!("  find_user(999) = {:?}", user);
    println!();
    
    // 何时使用 .ok_or()
    println!("何时使用 .ok_or():");
    println!("  ✓ 错误值是常量或字面量");
    println!("  ✓ 错误值计算开销很小");
    println!("  ✓ 简单的错误消息");
    println!("  ✓ None 必须被视为错误");
    println!();
    
    // 注意事项
    println!("⚠️ 注意: .ok_or() 总是计算错误值");
    println!("  即使 Option 是 Some，错误值也会被求值");
    println!("  如果错误值计算昂贵，使用 .ok_or_else()");
    println!();
}

// ============================================
// 4. .ok_or_else() 详解
// ============================================
fn demo_ok_or_else() {
    println!("--- 4. .ok_or_else() - 惰性求值 ---\n");
    
    println!("作用: 将 Option<T> 转换为 Result<T, E>（惰性）");
    println!("  - Some(v) → Ok(v)");
    println!("  - None    → Err(调用函数生成错误)");
    println!("  - 仅在 None 时才计算错误\n");
    
    // 基础示例
    println!("基础示例:");
    let some_value: Option<i32> = Some(100);
    let none_value: Option<i32> = None;
    
    println!("  Some(100).ok_or_else(|| '错误') = {:?}", 
             some_value.ok_or_else(|| "错误"));
    println!("  None.ok_or_else(|| '错误')      = {:?}", 
             none_value.ok_or_else(|| "错误"));
    println!();
    
    // 演示惰性求值
    println!("演示惰性求值:");
    
    fn expensive_error() -> String {
        println!("    → 正在计算昂贵的错误消息...");
        "昂贵的错误".to_string()
    }
    
    println!("\n  使用 .ok_or():");
    let some_opt: Option<i32> = Some(42);
    let _result = some_opt.ok_or(expensive_error());
    println!("    注意: 即使是 Some，expensive_error() 也被调用了");
    
    println!("\n  使用 .ok_or_else():");
    let some_opt: Option<i32> = Some(42);
    let _result = some_opt.ok_or_else(|| expensive_error());
    println!("    注意: 因为是 Some，expensive_error() 没有被调用");
    
    println!("\n  使用 .ok_or_else() 处理 None:");
    let none_opt: Option<i32> = None;
    let _result = none_opt.ok_or_else(|| expensive_error());
    println!();
    
    // 实际应用：动态错误消息
    println!("实际应用 1: 动态错误消息");
    use std::collections::HashMap;
    
    let users: HashMap<u32, String> = [
        (1, "Alice".to_string()),
        (2, "Bob".to_string()),
    ].iter().cloned().collect();
    
    fn find_user_with_error(users: &HashMap<u32, String>, id: u32) 
        -> Result<&String, String> 
    {
        users
            .get(&id)
            .ok_or_else(|| format!("用户 ID {} 不存在", id))
    }
    
    match find_user_with_error(&users, 1) {
        Ok(name) => println!("  找到: {}", name),
        Err(e) => println!("  错误: {}", e),
    }
    
    match find_user_with_error(&users, 999) {
        Ok(name) => println!("  找到: {}", name),
        Err(e) => println!("  错误: {}", e),
    }
    println!();
    
    // 实际应用：上下文错误
    println!("实际应用 2: 携带上下文的错误");
    
    #[derive(Debug)]
    struct Context {
        operation: String,
        timestamp: u64,
    }
    
    fn get_value() -> Option<i32> {
        None
    }
    
    let result = get_value().ok_or_else(|| Context {
        operation: "获取配置值".to_string(),
        timestamp: 1234567890,
    });
    
    println!("  结果: {:?}", result);
    println!("  说明: 错误包含丰富的上下文信息");
    println!();
    
    // 何时使用 .ok_or_else()
    println!("何时使用 .ok_or_else():");
    println!("  ✓ 错误值计算开销大");
    println!("  ✓ 需要动态生成错误消息");
    println!("  ✓ 错误需要包含上下文信息");
    println!("  ✓ 性能敏感的代码");
    println!();
}

// ============================================
// 5. 三者对比
// ============================================
fn demo_comparison() {
    println!("--- 5. 三者对比 ---\n");
    
    println!("┌──────────────┬─────────────┬─────────────┬─────────────┐");
    println!("│  方法         │  方向        │  错误处理    │  性能        │");
    println!("├──────────────┼─────────────┼─────────────┼─────────────┤");
    println!("│  .ok()       │  Result→Opt │  丢弃错误    │  无开销      │");
    println!("│  .ok_or()    │  Option→Res │  立即求值    │  总是求值    │");
    println!("│  .ok_or_else │  Option→Res │  惰性求值    │  按需求值    │");
    println!("└──────────────┴─────────────┴─────────────┴─────────────┘");
    println!();
    
    println!("使用场景对比:\n");
    
    // 场景 1: 解析配置
    println!("场景 1: 解析配置");
    use std::collections::HashMap;
    
    let config: HashMap<&str, &str> = [
        ("timeout", "30"),
    ].iter().cloned().collect();
    
    // 使用 .ok() - 可选配置
    let timeout1 = config.get("timeout")
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(60);
    println!("  .ok(): 可选配置 = {}", timeout1);
    
    // 使用 .ok_or() - 必需配置（简单错误）
    let timeout2 = config.get("timeout")
        .ok_or("缺少 timeout 配置")
        .and_then(|s| s.parse::<u32>()
            .map_err(|_| "timeout 格式错误"));
    println!("  .ok_or(): 必需配置 = {:?}", timeout2);
    
    // 使用 .ok_or_else() - 必需配置（详细错误）
    let timeout3 = config.get("timeout")
        .ok_or_else(|| format!("配置文件中缺少 'timeout' 字段"))
        .and_then(|s| s.parse::<u32>()
            .map_err(|e| format!("无法解析 timeout: {}", e)));
    println!("  .ok_or_else(): 详细错误 = {:?}", timeout3);
    println!();
    
    // 场景 2: 查找操作
    println!("场景 2: 数据库查找");
    
    fn find_in_db(id: u32) -> Option<String> {
        if id == 1 { Some("Data".to_string()) } else { None }
    }
    
    // Some/None - 不关心为什么找不到
    let data1 = find_in_db(1);
    println!("  直接 Option: {:?}", data1);
    
    // .ok_or() - 简单错误消息
    let data2 = find_in_db(2).ok_or("数据不存在");
    println!("  .ok_or(): {:?}", data2);
    
    // .ok_or_else() - 包含 ID 的错误消息
    let id = 3;
    let data3 = find_in_db(id).ok_or_else(|| 
        format!("ID {} 的数据不存在", id)
    );
    println!("  .ok_or_else(): {:?}", data3);
    println!();
    
    // 场景 3: 链式操作
    println!("场景 3: 链式操作");
    
    let compute_chain = || -> Result<i32, String> {
        let result = Some("42")
            .ok_or("值为空")?
            .parse::<i32>()
            .ok()
            .ok_or_else(|| "解析失败".to_string())?
            * 2;
        Ok(result)
    };
    
    println!("  链式结果: {:?}", compute_chain());
    println!();
}

// ============================================
// 6. 性能考虑
// ============================================
fn demo_performance() {
    println!("--- 6. 性能考虑 ---\n");
    
    println!("性能对比:\n");
    
    println!("1. .ok()");
    println!("   开销: 零");
    println!("   说明: 只是类型转换，不涉及计算");
    println!();
    
    println!("2. .ok_or(value)");
    println!("   开销: 总是求值 value");
    println!("   示例:");
    
    fn cheap_error() -> &'static str {
        "错误"  // 返回静态字符串，开销小
    }
    
    fn expensive_error() -> String {
        // 模拟昂贵的操作
        format!("时间: {:?}, 错误详情...", std::time::SystemTime::now())
    }
    
    println!("     let result = opt.ok_or(expensive_error());");
    println!("     ❌ 即使 opt 是 Some，expensive_error() 也会被调用");
    println!();
    
    println!("3. .ok_or_else(|| value)");
    println!("   开销: 仅在 None 时求值");
    println!("   示例:");
    println!("     let result = opt.ok_or_else(|| expensive_error());");
    println!("     ✅ 只有 opt 是 None 时才调用 expensive_error()");
    println!();
    
    println!("基准对比:");
    println!();
    println!("  场景: Some(value)");
    println!("  ┌────────────────┬──────────────┐");
    println!("  │  方法           │  开销         │");
    println!("  ├────────────────┼──────────────┤");
    println!("  │  .ok_or(exp()) │  100%        │");
    println!("  │  .ok_or_else() │  ~0%         │");
    println!("  └────────────────┴──────────────┘");
    println!();
    
    println!("  场景: None");
    println!("  ┌────────────────┬──────────────┐");
    println!("  │  方法           │  开销         │");
    println!("  ├────────────────┼──────────────┤");
    println!("  │  .ok_or(exp()) │  100%        │");
    println!("  │  .ok_or_else() │  100%        │");
    println!("  └────────────────┴──────────────┘");
    println!();
    
    println!("选择建议:");
    println!("  📌 常量/字面量 → 使用 .ok_or()");
    println!("  📌 函数调用     → 使用 .ok_or_else()");
    println!("  📌 String::new()→ 使用 .ok_or_else()");
    println!("  📌 format!()    → 使用 .ok_or_else()");
    println!();
}

// ============================================
// 7. 实战场景
// ============================================
fn demo_real_world_scenarios() {
    println!("--- 7. 实战场景 ---\n");
    
    // 场景 1: Web API 参数验证
    println!("场景 1: Web API 参数验证\n");
    api_validation_example();
    
    // 场景 2: 配置管理
    println!("\n场景 2: 配置文件管理\n");
    config_management_example();
    
    // 场景 3: 数据库操作
    println!("\n场景 3: 数据库查询\n");
    database_example();
}

fn api_validation_example() {
    use std::collections::HashMap;
    
    // 模拟 HTTP 请求参数
    let params: HashMap<&str, &str> = [
        ("user_id", "123"),
        ("action", "update"),
    ].iter().cloned().collect();
    
    fn validate_request(params: &HashMap<&str, &str>) 
        -> Result<(u32, String), String> 
    {
        // 必需参数 - 使用 .ok_or_else()（动态错误消息）
        let user_id = params
            .get("user_id")
            .ok_or_else(|| "缺少必需参数: user_id".to_string())?
            .parse::<u32>()
            .ok()  // 不关心解析错误的具体类型
            .ok_or_else(|| "user_id 必须是数字".to_string())?;
        
        // 必需参数 - 使用 .ok_or()（固定错误消息）
        let action = params
            .get("action")
            .ok_or("缺少必需参数: action")?
            .to_string();
        
        Ok((user_id, action))
    }
    
    match validate_request(&params) {
        Ok((id, action)) => {
            println!("  ✓ 验证成功");
            println!("    user_id: {}", id);
            println!("    action: {}", action);
        }
        Err(e) => println!("  ✗ 验证失败: {}", e),
    }
}

fn config_management_example() {
    use std::collections::HashMap;
    
    struct AppConfig {
        database_url: String,
        port: u16,
        log_level: String,
    }
    
    fn load_config(env_vars: &HashMap<&str, &str>) 
        -> Result<AppConfig, String> 
    {
        Ok(AppConfig {
            // 必需配置 - .ok_or_else()（详细错误）
            database_url: env_vars
                .get("DATABASE_URL")
                .ok_or_else(|| {
                    "DATABASE_URL 环境变量未设置。\n\
                     请设置: export DATABASE_URL=postgres://...".to_string()
                })?
                .to_string(),
            
            // 必需配置 - .ok_or()（简单错误）+ 解析
            port: env_vars
                .get("PORT")
                .ok_or("PORT 未设置")?
                .parse()
                .ok()  // 转换解析错误
                .ok_or("PORT 必须是有效的端口号")?,
            
            // 可选配置 - Option（提供默认值）
            log_level: env_vars
                .get("LOG_LEVEL")
                .map(|s| s.to_string())
                .unwrap_or_else(|| "info".to_string()),
        })
    }
    
    let env: HashMap<&str, &str> = [
        ("DATABASE_URL", "postgres://localhost/mydb"),
        ("PORT", "8080"),
    ].iter().cloned().collect();
    
    match load_config(&env) {
        Ok(config) => {
            println!("  ✓ 配置加载成功");
            println!("    database: {}", config.database_url);
            println!("    port: {}", config.port);
            println!("    log_level: {}", config.log_level);
        }
        Err(e) => println!("  ✗ 配置加载失败:\n{}", e),
    }
}

fn database_example() {
    use std::collections::HashMap;
    
    // 模拟数据库
    let users: HashMap<u32, String> = [
        (1, "Alice".to_string()),
        (2, "Bob".to_string()),
    ].iter().cloned().collect();
    
    fn find_user(users: &HashMap<u32, String>, id: u32) 
        -> Result<String, String> 
    {
        users
            .get(&id)
            .cloned()
            .ok_or_else(|| {
                format!(
                    "用户不存在: ID={}\n\
                     提示: 使用有效的用户 ID (1-{})",
                    id,
                    users.len()
                )
            })
    }
    
    // 成功查询
    match find_user(&users, 1) {
        Ok(name) => println!("  ✓ 找到用户: {}", name),
        Err(e) => println!("  ✗ {}", e),
    }
    
    // 失败查询 - 详细错误消息
    match find_user(&users, 999) {
        Ok(name) => println!("  ✓ 找到用户: {}", name),
        Err(e) => println!("  ✗ {}", e),
    }
}

// ============================================
// 8. 最佳实践
// ============================================
fn demo_best_practices() {
    println!("\n--- 8. 最佳实践和常见模式 ---\n");
    
    println!("📌 选择指南:\n");
    
    println!("使用 .ok() 当:");
    println!("  ✓ 将 Result 转换为 Option");
    println!("  ✓ 不关心错误的具体内容");
    println!("  ✓ 在 filter_map 中过滤");
    println!("  ✓ 链式操作中忽略错误");
    println!();
    println!("  示例:");
    println!("    let numbers: Vec<i32> = strings");
    println!("        .iter()");
    println!("        .filter_map(|s| s.parse().ok())");
    println!("        .collect();");
    println!();
    
    println!("使用 .ok_or() 当:");
    println!("  ✓ 错误是字面量或常量");
    println!("  ✓ 错误值计算开销很小");
    println!("  ✓ 固定的错误消息");
    println!();
    println!("  示例:");
    println!("    let value = option");
    println!("        .ok_or(\"值不能为空\")?;");
    println!();
    
    println!("使用 .ok_or_else() 当:");
    println!("  ✓ 需要动态生成错误消息");
    println!("  ✓ 错误包含上下文信息");
    println!("  ✓ 错误构造开销大（format!、String::new 等）");
    println!("  ✓ 性能敏感的代码");
    println!();
    println!("  示例:");
    println!("    let user = find_user(id)");
    println!("        .ok_or_else(|| format!(\"User {{}} not found\", id))?;");
    println!();
    
    println!("⚠️ 常见错误:\n");
    
    println!("错误 1: 在 .ok_or() 中使用昂贵的操作");
    println!("  ❌ option.ok_or(format!(\"Error: {{}}\", compute_details()))");
    println!("  ✅ option.ok_or_else(|| format!(\"Error: {{}}\", compute_details()))");
    println!();
    
    println!("错误 2: 过度使用 .unwrap()");
    println!("  ❌ option.ok_or(\"error\").unwrap()");
    println!("  ✅ option.ok_or(\"error\")?");
    println!();
    
    println!("错误 3: 链式调用中混淆 Result 和 Option");
    println!("  ❌ result.ok().ok_or(\"error\")  // 可能丢失原错误信息");
    println!("  ✅ result.map_err(|_| \"error\")  // 保留 Result 类型");
    println!();
    
    println!("🎯 记忆口诀:\n");
    println!("  Result 转 Option 用 .ok()");
    println!("  固定错误用 .ok_or()");
    println!("  动态错误用 .ok_or_else()");
    println!("  性能敏感选 .ok_or_else()");
    println!();
}

/*
=== 总结 ===

┌────────────────┬─────────────────────────────────────────┐
│  方法           │  使用场景                                │
├────────────────┼─────────────────────────────────────────┤
│  .ok()         │  • 不关心错误详情                        │
│                │  • filter_map 过滤                       │
│                │  • 可选配置                              │
├────────────────┼─────────────────────────────────────────┤
│  .ok_or()      │  • 简单的错误消息                        │
│                │  • 错误是常量/字面量                     │
│                │  • 计算开销小                            │
├────────────────┼─────────────────────────────────────────┤
│  .ok_or_else() │  • 动态错误消息                          │
│                │  • 包含上下文信息                        │
│                │  • format! 或昂贵的计算                  │
│                │  • 性能关键路径                          │
└────────────────┴─────────────────────────────────────────┘

核心原则:
  1. .ok()        → 简化类型，丢弃错误
  2. .ok_or()     → 简单固定错误
  3. .ok_or_else()→ 复杂动态错误

运行示例:
  cargo run --bin ok_methods_comparison
*/
