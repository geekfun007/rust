# .ok() vs .ok_or() vs .ok_or_else() 完全指南

## 🎯 快速对比

| 方法 | 输入 | 输出 | 作用 |
|------|------|------|------|
| `.ok()` | `Result<T, E>` | `Option<T>` | 丢弃错误，只保留值 |
| `.ok_or(err)` | `Option<T>` | `Result<T, E>` | 为 None 提供错误（立即求值） |
| `.ok_or_else(f)` | `Option<T>` | `Result<T, E>` | 为 None 提供错误（惰性求值） |

---

## 📖 详细说明

### 1. `.ok()` - 简化 Result

**类型签名**:
```rust
impl<T, E> Result<T, E> {
    fn ok(self) -> Option<T>
}
```

**转换规则**:
- `Ok(value)` → `Some(value)`
- `Err(_)` → `None`

**使用场景**:
```rust
// ✅ 场景 1: 过滤解析失败的值
let numbers: Vec<i32> = vec!["1", "2", "abc", "4"]
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();

// ✅ 场景 2: 可选配置
let timeout = env::var("TIMEOUT")
    .ok()
    .and_then(|s| s.parse::<u64>().ok())
    .unwrap_or(30);

// ✅ 场景 3: 忽略错误详情
let data = fetch_data().ok();
```

**何时使用**:
- ✅ 不关心错误的具体内容
- ✅ 只需要知道是否成功
- ✅ 配合 `filter_map` 过滤
- ✅ 链式调用中间步骤

---

### 2. `.ok_or(err)` - 添加固定错误

**类型签名**:
```rust
impl<T> Option<T> {
    fn ok_or<E>(self, err: E) -> Result<T, E>
}
```

**转换规则**:
- `Some(value)` → `Ok(value)`
- `None` → `Err(err)`

**使用场景**:
```rust
// ✅ 场景 1: 必需的配置
let host = config.get("host")
    .ok_or("缺少 host 配置")?;

// ✅ 场景 2: 简单查找
let user = find_user(id)
    .ok_or("用户不存在")?;

// ✅ 场景 3: 固定错误消息
fn get_value(opt: Option<i32>) -> Result<i32, &'static str> {
    opt.ok_or("值不能为空")
}
```

**何时使用**:
- ✅ 错误是字面量或常量
- ✅ 错误值计算开销很小
- ✅ 固定的错误消息
- ✅ None 必须被视为错误

**⚠️ 重要**: `.ok_or()` **总是**计算错误值，即使 Option 是 `Some`！

---

### 3. `.ok_or_else(f)` - 惰性错误生成

**类型签名**:
```rust
impl<T> Option<T> {
    fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
    where F: FnOnce() -> E
}
```

**转换规则**:
- `Some(value)` → `Ok(value)` (不调用 f)
- `None` → `Err(f())` (调用 f)

**使用场景**:
```rust
// ✅ 场景 1: 动态错误消息
let user = find_user(id)
    .ok_or_else(|| format!("用户 {} 不存在", id))?;

// ✅ 场景 2: 昂贵的错误构造
let config = parse_config()
    .ok_or_else(|| {
        // 这里的代码只在失败时执行
        log_error();
        compute_detailed_error_message()
    })?;

// ✅ 场景 3: 带上下文的错误
let value = get_value()
    .ok_or_else(|| Error {
        context: "配置加载",
        timestamp: SystemTime::now(),
    })?;
```

**何时使用**:
- ✅ 需要动态生成错误消息
- ✅ 错误构造开销大（`format!`、`String::new` 等）
- ✅ 错误需要包含上下文信息
- ✅ 性能敏感的代码

---

## 🚀 性能对比

### 基准测试结果

**场景 1: Option 是 Some**

```rust
let opt = Some(42);

// .ok_or() - 总是求值
opt.ok_or(expensive_computation())  // ❌ 计算了，但没用到

// .ok_or_else() - 不求值
opt.ok_or_else(|| expensive_computation())  // ✅ 没有计算
```

| 方法 | 开销 |
|------|------|
| `.ok_or(expensive())` | 100% |
| `.ok_or_else(\|\| expensive())` | ~0% |

**场景 2: Option 是 None**

| 方法 | 开销 |
|------|------|
| `.ok_or(expensive())` | 100% |
| `.ok_or_else(\|\| expensive())` | 100% |

### 性能建议

```rust
// ❌ 不好 - 总是计算
opt.ok_or(format!("Error: {}", details))

// ✅ 好 - 按需计算
opt.ok_or_else(|| format!("Error: {}", details))

// ✅ 更好 - 常量不需要惰性
opt.ok_or("Error: fixed message")
```

---

## 💡 实战模式

### 模式 1: 配置管理

```rust
struct Config {
    // 必需配置 - .ok_or_else()
    database_url: env::var("DATABASE_URL")
        .ok()
        .ok_or_else(|| {
            "DATABASE_URL 未设置。\n\
             请设置: export DATABASE_URL=...".to_string()
        })?,
    
    // 必需配置 - .ok_or()（简单消息）
    port: env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .ok_or("PORT 未设置或无效")?,
    
    // 可选配置 - .ok() + unwrap_or
    log_level: env::var("LOG_LEVEL")
        .ok()
        .unwrap_or_else(|| "info".to_string()),
}
```

### 模式 2: API 参数验证

```rust
fn validate_request(params: &HashMap<String, String>) 
    -> Result<Request, String> 
{
    Ok(Request {
        // 必需参数 + 动态错误
        user_id: params
            .get("user_id")
            .ok_or_else(|| format!("缺少参数: user_id"))?
            .parse()
            .ok()
            .ok_or_else(|| format!("user_id 必须是数字"))?,
        
        // 必需参数 + 固定错误
        action: params
            .get("action")
            .ok_or("缺少参数: action")?
            .clone(),
    })
}
```

### 模式 3: 数据库查询

```rust
fn find_user(db: &Database, id: u32) -> Result<User, DbError> {
    db.query_optional("SELECT * FROM users WHERE id = ?", id)
        .ok()  // Result -> Option (忽略查询错误)
        .flatten()  // Option<Option<User>> -> Option<User>
        .ok_or_else(|| DbError::NotFound {
            entity: "User",
            id: id.to_string(),
        })
}
```

---

## 📋 选择清单

### 使用 `.ok()` 当:
- [ ] 需要将 Result 转为 Option
- [ ] 不关心错误的具体内容
- [ ] 在 `filter_map` 中过滤失败项
- [ ] 链式操作中忽略中间错误

### 使用 `.ok_or()` 当:
- [ ] 错误是字符串字面量
- [ ] 错误是简单常量
- [ ] 错误值已经计算好了
- [ ] 代码简洁性优先于性能

### 使用 `.ok_or_else()` 当:
- [ ] 错误消息包含变量（需要 `format!`）
- [ ] 错误构造昂贵（分配、计算等）
- [ ] 错误需要上下文信息
- [ ] 在性能关键路径上
- [ ] 大多数情况下 Option 是 Some

---

## ⚠️ 常见错误

### 错误 1: 在 .ok_or() 中使用昂贵操作

```rust
// ❌ 错误 - 总是计算
config.get("key").ok_or(format!("Missing: {}", key))

// ✅ 正确
config.get("key").ok_or_else(|| format!("Missing: {}", key))
```

### 错误 2: 混淆 Result 和 Option

```rust
// ❌ 错误 - .ok() 后又 .ok_or()，可能丢失错误信息
result.ok().ok_or("error")

// ✅ 正确 - 保留 Result
result.map_err(|_| "error")
```

### 错误 3: 过度使用 .unwrap()

```rust
// ❌ 危险
let value = option.ok_or("error").unwrap();

// ✅ 安全
let value = option.ok_or("error")?;
```

---

## 🎯 记忆口诀

```
Result 转 Option 用 .ok()
固定错误用 .ok_or()
动态错误用 .ok_or_else()
性能敏感选 .ok_or_else()
```

---

## 🔗 相关资源

- **运行示例**: `cargo run --bin ok_methods_comparison`
- **源代码**: `src/ok_methods_comparison.rs`
- **相关教程**: `src/conversions.rs` (完整的类型转换教程)

---

## 📊 总结表格

| 特征 | .ok() | .ok_or() | .ok_or_else() |
|------|-------|----------|---------------|
| **输入** | Result | Option | Option |
| **输出** | Option | Result | Result |
| **错误处理** | 丢弃 | 立即求值 | 惰性求值 |
| **性能** | 最快 | Some 时浪费 | 最优 |
| **适用场景** | 简化类型 | 简单错误 | 复杂错误 |
| **典型用法** | filter_map | 常量错误 | format! |

**最后更新**: 2025-12-22
