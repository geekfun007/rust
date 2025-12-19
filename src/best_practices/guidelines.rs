// Rust 编程最佳实践与注意事项

/// # 代码组织
pub fn code_organization() {
    println!("\n=== 代码组织 ===");
    
    println!("1. 模块结构:");
    println!("   - 使用 mod.rs 组织模块");
    println!("   - 合理划分功能模块");
    println!("   - 避免循环依赖");
    
    println!("\n2. 文件结构:");
    println!("   project/");
    println!("   ├── src/");
    println!("   │   ├── main.rs 或 lib.rs");
    println!("   │   ├── module1/");
    println!("   │   │   ├── mod.rs");
    println!("   │   │   └── submodule.rs");
    println!("   │   └── module2.rs");
    println!("   ├── tests/           # 集成测试");
    println!("   ├── benches/         # 性能测试");
    println!("   └── examples/        # 示例代码");
    
    println!("\n3. 可见性:");
    println!("   - 默认私有");
    println!("   - pub 公开");
    println!("   - pub(crate) 包内公开");
    println!("   - pub(super) 父模块公开");
}

/// # 命名规范
pub fn naming_conventions() {
    println!("\n=== 命名规范 ===");
    
    println!("1. Rust 命名风格:");
    println!("   - 变量/函数: snake_case");
    println!("   - 类型/Trait: PascalCase");
    println!("   - 常量/静态: SCREAMING_SNAKE_CASE");
    println!("   - 包名: kebab-case");
    
    println!("\n2. 示例:");
    println!("   let user_name = \"Alice\";           // 变量");
    println!("   fn calculate_sum() {{ }}            // 函数");
    println!("   struct UserAccount {{ }}            // 结构体");
    println!("   trait Drawable {{ }}                // trait");
    println!("   const MAX_SIZE: usize = 100;       // 常量");
    
    println!("\n3. 命名建议:");
    println!("   - 使用描述性名称");
    println!("   - 避免缩写");
    println!("   - 遵循领域术语");
    println!("   - 保持一致性");
}

/// # 错误处理
pub fn error_handling_best_practices() {
    println!("\n=== 错误处理最佳实践 ===");
    
    println!("1. 使用 Result:");
    println!("   - 可恢复错误用 Result");
    println!("   - 不可恢复错误用 panic!");
    println!("   - 避免在库代码中 panic");
    
    println!("\n2. 错误传播:");
    println!("   - 使用 ? 操作符");
    println!("   - 提供上下文信息");
    println!("   - 自定义错误类型");
    
    println!("\n3. 错误类型设计:");
    println!("   - 库代码: 使用 thiserror");
    println!("   - 应用代码: 使用 anyhow");
    println!("   - 实现 std::error::Error");
    
    println!("\n4. 避免:");
    println!("   - 过度使用 unwrap()");
    println!("   - 忽略错误");
    println!("   - 使用空字符串作为错误消息");
}

/// # 性能优化
pub fn performance_tips() {
    println!("\n=== 性能优化 ===");
    
    println!("1. 所有权:");
    println!("   - 优先使用借用");
    println!("   - 避免不必要的克隆");
    println!("   - 使用 Cow<T> 处理可能拥有的数据");
    
    println!("\n2. 集合:");
    println!("   - 预分配容量 with_capacity()");
    println!("   - 选择合适的集合类型");
    println!("   - 使用迭代器而不是索引");
    
    println!("\n3. 字符串:");
    println!("   - 使用 String::with_capacity");
    println!("   - 参数使用 &str 而不是 &String");
    println!("   - 避免频繁的字符串连接");
    
    println!("\n4. 并发:");
    println!("   - 使用 Rayon 进行数据并行");
    println!("   - 异步 I/O 使用 tokio");
    println!("   - 避免过度锁定");
    
    println!("\n5. 编译器优化:");
    println!("   - 发布模式编译");
    println!("   - 使用 #[inline]");
    println!("   - LTO (Link Time Optimization)");
}

/// # 安全编程
pub fn safety_guidelines() {
    println!("\n=== 安全编程 ===");
    
    println!("1. 避免 unsafe:");
    println!("   - 尽可能不用 unsafe");
    println!("   - 必要时限制 unsafe 范围");
    println!("   - 在 unsafe 块前注释说明原因");
    
    println!("\n2. 数据验证:");
    println!("   - 验证所有外部输入");
    println!("   - 使用类型系统保证不变量");
    println!("   - 边界检查");
    
    println!("\n3. 资源管理:");
    println!("   - 使用 RAII 模式");
    println!("   - 避免资源泄漏");
    println!("   - 正确处理生命周期");
    
    println!("\n4. 并发安全:");
    println!("   - 理解 Send 和 Sync");
    println!("   - 使用类型系统防止数据竞争");
    println!("   - 避免死锁");
}

/// # 测试
pub fn testing_guidelines() {
    println!("\n=== 测试指南 ===");
    
    println!("1. 单元测试:");
    println!("   #[cfg(test)]");
    println!("   mod tests {{");
    println!("       use super::*;");
    println!("       ");
    println!("       #[test]");
    println!("       fn test_function() {{");
    println!("           assert_eq!(add(2, 3), 5);");
    println!("       }}");
    println!("   }}");
    
    println!("\n2. 集成测试:");
    println!("   - 放在 tests/ 目录");
    println!("   - 测试公共 API");
    println!("   - 每个文件是独立的 crate");
    
    println!("\n3. 文档测试:");
    println!("   /// ```");
    println!("   /// let result = add(2, 3);");
    println!("   /// assert_eq!(result, 5);");
    println!("   /// ```");
    println!("   pub fn add(a: i32, b: i32) -> i32 {{ a + b }}");
    
    println!("\n4. 测试最佳实践:");
    println!("   - 测试覆盖率");
    println!("   - 测试边界条件");
    println!("   - 测试错误情况");
    println!("   - 使用描述性测试名");
}

/// # 文档
pub fn documentation_guidelines() {
    println!("\n=== 文档指南 ===");
    
    println!("1. 文档注释:");
    println!("   /// 函数级文档");
    println!("   /// ");
    println!("   /// # Examples");
    println!("   /// ```");
    println!("   /// let result = function();");
    println!("   /// ```");
    println!("   pub fn function() {{ }}");
    
    println!("\n2. 模块文档:");
    println!("   //! 模块级文档");
    println!("   //! ");
    println!("   //! 这个模块提供...");
    
    println!("\n3. README:");
    println!("   - 项目描述");
    println!("   - 安装说明");
    println!("   - 使用示例");
    println!("   - API 文档链接");
    
    println!("\n4. 生成文档:");
    println!("   cargo doc --open");
}

/// # 依赖管理
pub fn dependency_management() {
    println!("\n=== 依赖管理 ===");
    
    println!("1. Cargo.toml 配置:");
    println!("   [dependencies]");
    println!("   serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("   ");
    println!("   [dev-dependencies]");
    println!("   mockall = \"0.11\"");
    
    println!("\n2. 版本选择:");
    println!("   - 使用语义化版本");
    println!("   - ^1.2.3: 兼容 1.x.x");
    println!("   - ~1.2.3: 兼容 1.2.x");
    println!("   - 固定版本: =1.2.3");
    
    println!("\n3. 更新依赖:");
    println!("   cargo update          # 更新依赖");
    println!("   cargo outdated        # 检查过时依赖");
    println!("   cargo audit           # 安全审计");
    
    println!("\n4. 最小化依赖:");
    println!("   - 只添加必要的依赖");
    println!("   - 考虑编译时间");
    println!("   - 使用 features 控制功能");
}

/// # 代码风格
pub fn code_style() {
    println!("\n=== 代码风格 ===");
    
    println!("1. 使用 rustfmt:");
    println!("   rustfmt src/**/*.rs");
    println!("   cargo fmt");
    
    println!("\n2. 使用 clippy:");
    println!("   cargo clippy");
    println!("   cargo clippy -- -D warnings");
    
    println!("\n3. 编辑器配置:");
    println!("   - 安装 rust-analyzer");
    println!("   - 配置自动格式化");
    println!("   - 启用 clippy 检查");
    
    println!("\n4. 团队规范:");
    println!("   - 统一的 rustfmt.toml");
    println!("   - CI/CD 检查格式");
    println!("   - 代码审查");
}

/// # 常见陷阱
pub fn common_pitfalls() {
    println!("\n=== 常见陷阱 ===");
    
    println!("1. 所有权:");
    println!("   ❌ let s1 = String::from(\"hello\");");
    println!("   ❌ let s2 = s1;  // s1 被移动");
    println!("   ❌ println!(\"{{}}\", s1);  // 错误！");
    println!("   ");
    println!("   ✓ let s1 = String::from(\"hello\");");
    println!("   ✓ let s2 = s1.clone();  // 克隆");
    println!("   ✓ println!(\"{{}}\", s1);  // 正确");
    
    println!("\n2. 借用:");
    println!("   ❌ let mut v = vec![1, 2, 3];");
    println!("   ❌ let first = &v[0];");
    println!("   ❌ v.push(4);  // 错误！可变借用冲突");
    println!("   ");
    println!("   ✓ let mut v = vec![1, 2, 3];");
    println!("   ✓ {{");
    println!("   ✓     let first = &v[0];");
    println!("   ✓     println!(\"{{}}\", first);");
    println!("   ✓ }}");
    println!("   ✓ v.push(4);  // 正确");
    
    println!("\n3. 整数溢出:");
    println!("   ❌ let x: u8 = 255;");
    println!("   ❌ let y = x + 1;  // Debug 模式 panic");
    println!("   ");
    println!("   ✓ let x: u8 = 255;");
    println!("   ✓ let y = x.checked_add(1).unwrap_or(0);");
    
    println!("\n4. 字符串切片:");
    println!("   ❌ let s = \"你好\";");
    println!("   ❌ let slice = &s[0..1];  // 错误！不在字符边界");
    println!("   ");
    println!("   ✓ let s = \"你好\";");
    println!("   ✓ let slice = &s[0..3];  // UTF-8 每个汉字3字节");
}

/// # 项目结构
pub fn project_structure() {
    println!("\n=== 项目结构建议 ===");
    
    println!("小型项目:");
    println!("  src/");
    println!("  ├── main.rs");
    println!("  ├── lib.rs");
    println!("  ├── config.rs");
    println!("  └── utils.rs");
    
    println!("\n中型项目:");
    println!("  src/");
    println!("  ├── main.rs");
    println!("  ├── lib.rs");
    println!("  ├── api/");
    println!("  │   ├── mod.rs");
    println!("  │   ├── routes.rs");
    println!("  │   └── handlers.rs");
    println!("  ├── db/");
    println!("  │   ├── mod.rs");
    println!("  │   └── models.rs");
    println!("  └── utils/");
    println!("      ├── mod.rs");
    println!("      └── helpers.rs");
    
    println!("\n大型项目:");
    println!("  - 考虑 Workspace");
    println!("  - 拆分为多个 crate");
    println!("  - 共享公共代码");
}

/// # CI/CD
pub fn cicd_practices() {
    println!("\n=== CI/CD 最佳实践 ===");
    
    println!("GitHub Actions 示例:");
    println!("  name: CI");
    println!("  on: [push, pull_request]");
    println!("  jobs:");
    println!("    test:");
    println!("      runs-on: ubuntu-latest");
    println!("      steps:");
    println!("        - uses: actions/checkout@v2");
    println!("        - uses: actions-rs/toolchain@v1");
    println!("        - run: cargo test");
    println!("        - run: cargo clippy");
    println!("        - run: cargo fmt -- --check");
    
    println!("\n检查项:");
    println!("  ✓ 单元测试");
    println!("  ✓ 集成测试");
    println!("  ✓ Clippy 检查");
    println!("  ✓ 格式检查");
    println!("  ✓ 安全审计");
}

/// # 发布
pub fn release_guidelines() {
    println!("\n=== 发布指南 ===");
    
    println!("1. 版本号:");
    println!("   - 语义化版本: MAJOR.MINOR.PATCH");
    println!("   - 0.x.x: 开发阶段");
    println!("   - 1.0.0: 稳定版本");
    
    println!("\n2. 发布到 crates.io:");
    println!("   cargo login");
    println!("   cargo publish --dry-run");
    println!("   cargo publish");
    
    println!("\n3. 发布检查清单:");
    println!("   ✓ 更新版本号");
    println!("   ✓ 更新 CHANGELOG");
    println!("   ✓ 运行所有测试");
    println!("   ✓ 更新文档");
    println!("   ✓ Git 标签");
}

/// 运行所有最佳实践示例
pub fn run_all() {
    println!("\n╔════════════════════════════════════╗");
    println!("║    Rust 编程最佳实践与注意事项     ║");
    println!("╚════════════════════════════════════╝");
    
    code_organization();
    naming_conventions();
    error_handling_best_practices();
    performance_tips();
    safety_guidelines();
    testing_guidelines();
    documentation_guidelines();
    dependency_management();
    code_style();
    common_pitfalls();
    project_structure();
    cicd_practices();
    release_guidelines();
}
