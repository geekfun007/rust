// Anyhow 详解 - 完全指南
//
// anyhow 是 Rust 中最流行的应用层错误处理库
// 本教程涵盖所有核心概念和实战技巧

use anyhow::{anyhow, bail, ensure, Context, Result};
use std::fs;
use std::io;

fn main() -> Result<()> {
    println!("=== Anyhow 错误处理详解 ===\n");
    
    // 1. Anyhow 基础
    demo_anyhow_basics()?;
    
    // 2. Result<T> 类型
    demo_result_type()?;
    
    // 3. Context - 添加上下文
    demo_context()?;
    
    // 4. bail! 宏
    demo_bail_macro()?;
    
    // 5. ensure! 宏
    demo_ensure_macro()?;
    
    // 6. anyhow! 宏
    demo_anyhow_macro()?;
    
    // 7. 错误转换
    demo_error_conversion()?;
    
    // 8. 错误链和回溯
    demo_error_chain()?;
    
    // 9. 实战案例
    demo_real_world_examples()?;
    
    // 10. 最佳实践
    demo_best_practices()?;
    
    println!("\n✅ 所有示例运行成功！");
    Ok(())
}

// ============================================
// 1. Anyhow 基础
// ============================================
fn demo_anyhow_basics() -> Result<()> {
    println!("--- 1. Anyhow 基础 ---\n");
    
    println!("什么是 Anyhow？");
    println!("  - 简化应用程序的错误处理");
    println!("  - 统一的错误类型 anyhow::Error");
    println!("  - 自动类型转换（任何实现了 Error 的类型）");
    println!("  - 丰富的错误上下文");
    println!("  - 回溯支持\n");
    
    println!("核心类型:");
    println!("  📌 anyhow::Result<T>  = Result<T, anyhow::Error>");
    println!("  📌 anyhow::Error      = 统一错误类型");
    println!("  📌 Context trait      = 添加上下文信息");
    println!();
    
    println!("核心宏:");
    println!("  📌 bail!(msg)         = 立即返回错误");
    println!("  📌 ensure!(cond, msg) = 条件检查");
    println!("  📌 anyhow!(msg)       = 创建错误对象");
    println!();
    
    println!("使用场景:");
    println!("  ✓ 应用程序（CLI、服务器等）");
    println!("  ✓ 快速原型开发");
    println!("  ✓ 多种错误类型混合");
    println!("  ✗ 库开发（推荐使用 thiserror）\n");
    
    Ok(())
}

// ============================================
// 2. Result<T> 类型
// ============================================
fn demo_result_type() -> Result<()> {
    println!("--- 2. Result<T> 类型 ---\n");
    
    println!("标准 Result vs Anyhow Result:\n");
    
    println!("标准库:");
    println!("  Result<T, E> - 需要指定具体错误类型");
    println!("  fn process() -> Result<String, std::io::Error>");
    println!();
    
    println!("Anyhow:");
    println!("  Result<T> = Result<T, anyhow::Error>");
    println!("  fn process() -> Result<String>  // 简洁！");
    println!();
    
    // 示例：简化函数签名
    println!("示例: 简化的函数签名");
    
    fn read_file_content(path: &str) -> Result<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }
    
    // 测试（文件不存在会返回错误）
    match read_file_content("nonexistent.txt") {
        Ok(_) => println!("  文件读取成功"),
        Err(e) => println!("  预期错误: {}", e),
    }
    println!();
    
    // 自动类型转换
    println!("自动类型转换:");
    
    fn mixed_errors() -> Result<()> {
        // io::Error 自动转换
        let _content = fs::read_to_string("test.txt")?;
        
        // ParseIntError 自动转换
        let _num: i32 = "not_a_number".parse()?;
        
        Ok(())
    }
    
    match mixed_errors() {
        Ok(_) => println!("  成功"),
        Err(e) => println!("  错误: {} (自动转换)", e),
    }
    println!();
    
    Ok(())
}

// ============================================
// 3. Context - 添加上下文
// ============================================
fn demo_context() -> Result<()> {
    println!("--- 3. Context - 添加上下文 ---\n");
    
    println!("为什么需要上下文？");
    println!("  原始错误: 'No such file or directory'");
    println!("  带上下文: 'Failed to read config file at /etc/app.conf: No such file or directory'\n");
    
    // 不使用 context
    println!("不使用 Context:");
    
    fn read_without_context(path: &str) -> Result<String> {
        let content = fs::read_to_string(path)?;
        Ok(content)
    }
    
    match read_without_context("missing.txt") {
        Err(e) => println!("  错误: {}", e),
        _ => {}
    }
    println!();
    
    // 使用 context
    println!("使用 Context:");
    
    fn read_with_context(path: &str) -> Result<String> {
        let content = fs::read_to_string(path)
            .context("Failed to read configuration file")?;
        Ok(content)
    }
    
    match read_with_context("missing.txt") {
        Err(e) => println!("  错误: {}", e),
        _ => {}
    }
    println!();
    
    // with_context - 动态上下文
    println!("with_context - 动态上下文:");
    
    fn read_with_dynamic_context(path: &str) -> Result<String> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path))?;
        Ok(content)
    }
    
    match read_with_dynamic_context("/etc/missing.conf") {
        Err(e) => println!("  错误: {}", e),
        _ => {}
    }
    println!();
    
    // 链式添加上下文
    println!("链式添加上下文:");
    
    fn parse_config(path: &str) -> Result<i32> {
        let content = fs::read_to_string(path)
            .context("Failed to read file")?;
        
        let value: i32 = content.trim().parse()
            .context("Failed to parse number")?;
        
        Ok(value)
    }
    
    match parse_config("missing.txt") {
        Err(e) => println!("  错误链: {}", e),
        _ => {}
    }
    println!();
    
    Ok(())
}

// ============================================
// 4. bail! 宏
// ============================================
fn demo_bail_macro() -> Result<()> {
    println!("--- 4. bail! 宏 ---\n");
    
    println!("作用: 立即返回错误");
    println!("语法: bail!(\"error message\")\n");
    
    // 基础用法
    println!("基础用法:");
    
    fn validate_age(age: i32) -> Result<()> {
        if age < 0 {
            bail!("年龄不能为负数");
        }
        if age > 150 {
            bail!("年龄不能超过 150");
        }
        Ok(())
    }
    
    match validate_age(-5) {
        Err(e) => println!("  验证失败: {}", e),
        _ => {}
    }
    
    match validate_age(200) {
        Err(e) => println!("  验证失败: {}", e),
        _ => {}
    }
    println!();
    
    // 格式化消息
    println!("格式化消息:");
    
    fn check_balance(balance: f64, amount: f64) -> Result<()> {
        if amount > balance {
            bail!("余额不足: 需要 {}, 当前 {}", amount, balance);
        }
        Ok(())
    }
    
    match check_balance(100.0, 150.0) {
        Err(e) => println!("  {}", e),
        _ => {}
    }
    println!();
    
    // bail! vs return Err
    println!("bail! vs return Err:");
    println!("  bail!(\"error\")");
    println!("  等价于");
    println!("  return Err(anyhow!(\"error\"))");
    println!();
    
    Ok(())
}

// ============================================
// 5. ensure! 宏
// ============================================
fn demo_ensure_macro() -> Result<()> {
    println!("--- 5. ensure! 宏 ---\n");
    
    println!("作用: 条件断言，不满足则返回错误");
    println!("语法: ensure!(condition, \"error message\")\n");
    
    // 基础用法
    println!("基础用法:");
    
    fn divide(a: f64, b: f64) -> Result<f64> {
        ensure!(b != 0.0, "除数不能为零");
        Ok(a / b)
    }
    
    match divide(10.0, 0.0) {
        Err(e) => println!("  错误: {}", e),
        _ => {}
    }
    
    match divide(10.0, 2.0) {
        Ok(result) => println!("  10 / 2 = {}", result),
        _ => {}
    }
    println!();
    
    // 复杂条件
    println!("复杂条件检查:");
    
    fn validate_username(username: &str) -> Result<()> {
        ensure!(!username.is_empty(), "用户名不能为空");
        ensure!(username.len() >= 3, "用户名至少 3 个字符");
        ensure!(username.len() <= 20, "用户名最多 20 个字符");
        ensure!(username.chars().all(|c| c.is_alphanumeric() || c == '_'), 
                "用户名只能包含字母、数字和下划线");
        Ok(())
    }
    
    let test_names = vec!["", "ab", "valid_user123", "invalid@user"];
    
    for name in test_names {
        match validate_username(name) {
            Ok(_) => println!("  '{}' 验证通过", name),
            Err(e) => println!("  '{}' 验证失败: {}", name, e),
        }
    }
    println!();
    
    // ensure! vs if + bail!
    println!("ensure! vs if + bail!:");
    println!("  ensure!(x > 0, \"x must be positive\");");
    println!("  等价于");
    println!("  if !(x > 0) {{ bail!(\"x must be positive\"); }}");
    println!();
    
    Ok(())
}

// ============================================
// 6. anyhow! 宏
// ============================================
fn demo_anyhow_macro() -> Result<()> {
    println!("--- 6. anyhow! 宏 ---\n");
    
    println!("作用: 创建错误对象（不立即返回）");
    println!("语法: anyhow!(\"error message\")\n");
    
    // 基础用法
    println!("基础用法:");
    
    fn process_data(data: &str) -> Result<i32> {
        if data.is_empty() {
            // 创建错误对象并返回
            return Err(anyhow!("数据为空"));
        }
        
        data.parse().context("解析失败")
    }
    
    match process_data("") {
        Err(e) => println!("  {}", e),
        _ => {}
    }
    println!();
    
    // 存储错误
    println!("存储错误对象:");
    
    fn collect_errors() {
        let mut errors = Vec::new();
        
        // 收集多个错误
        if true {
            errors.push(anyhow!("第一个错误"));
        }
        if true {
            errors.push(anyhow!("第二个错误"));
        }
        
        println!("  收集到 {} 个错误:", errors.len());
        for (i, err) in errors.iter().enumerate() {
            println!("    {}. {}", i + 1, err);
        }
    }
    
    collect_errors();
    println!();
    
    // 格式化错误
    println!("格式化错误信息:");
    
    fn validate_range(value: i32, min: i32, max: i32) -> Result<()> {
        if value < min || value > max {
            return Err(anyhow!(
                "值 {} 超出范围 [{}, {}]",
                value, min, max
            ));
        }
        Ok(())
    }
    
    match validate_range(150, 0, 100) {
        Err(e) => println!("  {}", e),
        _ => {}
    }
    println!();
    
    Ok(())
}

// ============================================
// 7. 错误转换
// ============================================
fn demo_error_conversion() -> Result<()> {
    println!("--- 7. 错误转换 ---\n");
    
    println!("Anyhow 自动转换任何实现了 std::error::Error 的类型\n");
    
    // 标准库错误
    println!("标准库错误自动转换:");
    
    fn mix_io_and_parse() -> Result<()> {
        // io::Error 自动转换
        let _content = fs::read_to_string("test.txt")?;
        
        // ParseIntError 自动转换
        let _num: i32 = "123".parse()?;
        
        Ok(())
    }
    
    match mix_io_and_parse() {
        Ok(_) => println!("  成功"),
        Err(e) => println!("  错误: {}", e),
    }
    println!();
    
    // 自定义错误
    println!("自定义错误转换:");
    
    #[derive(Debug)]
    struct CustomError {
        message: String,
    }
    
    impl std::fmt::Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "CustomError: {}", self.message)
        }
    }
    
    impl std::error::Error for CustomError {}
    
    fn use_custom_error() -> Result<()> {
        // 自定义错误自动转换为 anyhow::Error
        Err(CustomError {
            message: "Something went wrong".to_string(),
        })?;
        Ok(())
    }
    
    match use_custom_error() {
        Err(e) => println!("  {}", e),
        _ => {}
    }
    println!();
    
    // 字符串转错误
    println!("字符串转错误:");
    
    fn string_to_error() -> Result<()> {
        Err(anyhow!("简单的字符串错误"))
    }
    
    match string_to_error() {
        Err(e) => println!("  {}", e),
        _ => {}
    }
    println!();
    
    Ok(())
}

// ============================================
// 8. 错误链和回溯
// ============================================
fn demo_error_chain() -> Result<()> {
    println!("--- 8. 错误链和回溯 ---\n");
    
    println!("错误链: 多层上下文信息\n");
    
    // 创建错误链
    fn level_3() -> Result<()> {
        fs::read_to_string("missing.txt")
            .context("Level 3: 读取文件失败")?;
        Ok(())
    }
    
    fn level_2() -> Result<()> {
        level_3().context("Level 2: 配置加载失败")?;
        Ok(())
    }
    
    fn level_1() -> Result<()> {
        level_2().context("Level 1: 应用初始化失败")?;
        Ok(())
    }
    
    match level_1() {
        Err(e) => {
            println!("错误信息:");
            println!("  {}", e);
            println!();
            
            println!("错误链:");
            for (i, cause) in e.chain().enumerate() {
                println!("  {}. {}", i, cause);
            }
        }
        _ => {}
    }
    println!();
    
    // 根本原因
    println!("获取根本原因:");
    
    match level_1() {
        Err(e) => {
            if let Some(root) = e.root_cause().downcast_ref::<io::Error>() {
                println!("  根本原因类型: io::Error");
                println!("  错误种类: {:?}", root.kind());
            }
        }
        _ => {}
    }
    println!();
    
    println!("回溯 (Backtrace):");
    println!("  设置环境变量: RUST_BACKTRACE=1");
    println!("  运行程序查看完整调用栈");
    println!();
    
    Ok(())
}

// ============================================
// 9. 实战案例
// ============================================
fn demo_real_world_examples() -> Result<()> {
    println!("--- 9. 实战案例 ---\n");
    
    // 案例 1: 配置文件加载
    println!("案例 1: 配置文件加载\n");
    config_loader_example()?;
    
    // 案例 2: 数据处理管道
    println!("\n案例 2: 数据处理管道\n");
    data_pipeline_example()?;
    
    // 案例 3: CLI 工具
    println!("\n案例 3: CLI 工具\n");
    cli_tool_example()?;
    
    Ok(())
}

// 案例 1: 配置文件加载
fn config_loader_example() -> Result<()> {
    #[derive(Debug)]
    struct Config {
        host: String,
        port: u16,
        timeout: u64,
    }
    
    fn load_config(path: &str) -> Result<Config> {
        // 读取文件
        let content = fs::read_to_string(path)
            .with_context(|| format!("无法读取配置文件: {}", path))?;
        
        // 解析行
        let mut host = None;
        let mut port = None;
        let mut timeout = None;
        
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let parts: Vec<&str> = line.split('=').collect();
            ensure!(
                parts.len() == 2,
                "配置文件第 {} 行格式错误: {}",
                line_num + 1,
                line
            );
            
            let key = parts[0].trim();
            let value = parts[1].trim();
            
            match key {
                "host" => host = Some(value.to_string()),
                "port" => {
                    port = Some(
                        value.parse()
                            .with_context(|| format!("无效的端口号: {}", value))?
                    );
                }
                "timeout" => {
                    timeout = Some(
                        value.parse()
                            .with_context(|| format!("无效的超时值: {}", value))?
                    );
                }
                _ => bail!("未知的配置项: {}", key),
            }
        }
        
        // 验证必需字段
        let host = host.ok_or_else(|| anyhow!("缺少必需的配置项: host"))?;
        let port = port.ok_or_else(|| anyhow!("缺少必需的配置项: port"))?;
        let timeout = timeout.unwrap_or(30); // 默认值
        
        Ok(Config { host, port, timeout })
    }
    
    // 模拟配置文件
    let config_content = "\
host = localhost
port = 8080
timeout = 60
";
    
    // 写入临时文件
    fs::write("/tmp/app_config.txt", config_content)
        .context("写入临时配置文件失败")?;
    
    // 加载配置
    match load_config("/tmp/app_config.txt") {
        Ok(config) => {
            println!("  配置加载成功:");
            println!("    主机: {}", config.host);
            println!("    端口: {}", config.port);
            println!("    超时: {}s", config.timeout);
        }
        Err(e) => {
            println!("  配置加载失败:");
            for (i, cause) in e.chain().enumerate() {
                println!("    {}. {}", i + 1, cause);
            }
        }
    }
    
    // 清理
    let _ = fs::remove_file("/tmp/app_config.txt");
    
    Ok(())
}

// 案例 2: 数据处理管道
fn data_pipeline_example() -> Result<()> {
    #[derive(Debug)]
    struct Record {
        id: u32,
        name: String,
        score: f64,
    }
    
    fn parse_csv_line(line: &str, line_num: usize) -> Result<Record> {
        let parts: Vec<&str> = line.split(',').collect();
        
        ensure!(
            parts.len() == 3,
            "第 {} 行: 期望 3 列，实际 {} 列",
            line_num,
            parts.len()
        );
        
        let id: u32 = parts[0].trim().parse()
            .with_context(|| format!("第 {} 行: 无效的 ID '{}'", line_num, parts[0]))?;
        
        let name = parts[1].trim();
        ensure!(
            !name.is_empty(),
            "第 {} 行: 姓名不能为空",
            line_num
        );
        
        let score: f64 = parts[2].trim().parse()
            .with_context(|| format!("第 {} 行: 无效的分数 '{}'", line_num, parts[2]))?;
        
        ensure!(
            (0.0..=100.0).contains(&score),
            "第 {} 行: 分数 {} 超出范围 [0, 100]",
            line_num,
            score
        );
        
        Ok(Record {
            id,
            name: name.to_string(),
            score,
        })
    }
    
    fn process_csv(content: &str) -> Result<Vec<Record>> {
        let mut records = Vec::new();
        
        for (i, line) in content.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            
            let record = parse_csv_line(line, i + 1)
                .context("CSV 解析失败")?;
            records.push(record);
        }
        
        Ok(records)
    }
    
    // 测试数据
    let csv_data = "\
1, Alice, 95.5
2, Bob, 87.0
3, Charlie, 92.3
";
    
    match process_csv(csv_data) {
        Ok(records) => {
            println!("  成功解析 {} 条记录:", records.len());
            for record in records {
                println!("    {:?}", record);
            }
        }
        Err(e) => {
            println!("  处理失败:");
            for cause in e.chain() {
                println!("    - {}", cause);
            }
        }
    }
    
    Ok(())
}

// 案例 3: CLI 工具
fn cli_tool_example() -> Result<()> {
    fn process_command(cmd: &str, args: &[&str]) -> Result<()> {
        match cmd {
            "add" => {
                ensure!(args.len() == 2, "add 命令需要 2 个参数");
                
                let a: i32 = args[0].parse()
                    .context("第一个参数必须是整数")?;
                let b: i32 = args[1].parse()
                    .context("第二个参数必须是整数")?;
                
                println!("    结果: {} + {} = {}", a, b, a + b);
            }
            "div" => {
                ensure!(args.len() == 2, "div 命令需要 2 个参数");
                
                let a: f64 = args[0].parse()
                    .context("第一个参数必须是数字")?;
                let b: f64 = args[1].parse()
                    .context("第二个参数必须是数字")?;
                
                ensure!(b != 0.0, "除数不能为零");
                
                println!("    结果: {} / {} = {}", a, b, a / b);
            }
            "greet" => {
                ensure!(!args.is_empty(), "greet 命令需要至少 1 个参数");
                println!("    Hello, {}!", args.join(" "));
            }
            _ => bail!("未知命令: {}", cmd),
        }
        
        Ok(())
    }
    
    // 测试命令
    let commands = vec![
        ("add", vec!["10", "20"]),
        ("div", vec!["100", "5"]),
        ("greet", vec!["Alice"]),
        ("unknown", vec![]),
    ];
    
    for (cmd, args) in commands {
        println!("  执行: {} {:?}", cmd, args);
        match process_command(cmd, &args) {
            Ok(_) => {}
            Err(e) => println!("    错误: {}", e),
        }
    }
    
    Ok(())
}

// ============================================
// 10. 最佳实践
// ============================================
fn demo_best_practices() -> Result<()> {
    println!("--- 10. 最佳实践 ---\n");
    
    println!("1. 使用 context 添加上下文信息");
    println!("   ✅ .context(\"读取配置文件失败\")");
    println!("   ❌ 直接 ?");
    println!();
    
    println!("2. 使用 with_context 添加动态信息");
    println!("   ✅ .with_context(|| format!(\"文件: {{}}\", path))");
    println!("   ❌ .context(format!(\"文件: {{}}\", path))  // 总是求值");
    println!();
    
    println!("3. 使用 ensure! 进行条件检查");
    println!("   ✅ ensure!(x > 0, \"x 必须为正数\")");
    println!("   ❌ if x <= 0 {{ bail!(\"...\") }}");
    println!();
    
    println!("4. 使用 bail! 提前返回");
    println!("   ✅ bail!(\"错误信息\")");
    println!("   ❌ return Err(anyhow!(\"错误信息\"))");
    println!();
    
    println!("5. 为错误添加足够的上下文");
    println!("   ✅ 多层 context，形成错误链");
    println!("   ❌ 只返回底层错误");
    println!();
    
    println!("6. 使用 Result<()> 作为 main 返回类型");
    println!("   ✅ fn main() -> Result<()>");
    println!("   ✅ 错误会自动打印到 stderr");
    println!();
    
    println!("7. 库开发使用 thiserror，应用开发使用 anyhow");
    println!("   - thiserror: 定义具体的错误类型");
    println!("   - anyhow: 简化错误处理");
    println!();
    
    println!("8. 不要过度使用 anyhow");
    println!("   ✅ 应用层代码");
    println!("   ❌ 公共 API（库）");
    println!("   ❌ 需要精确错误匹配的场景");
    println!();
    
    // 示例：良好的错误处理
    println!("示例：良好的错误处理\n");
    
    fn good_error_handling_example() -> Result<()> {
        // 1. 使用 with_context 添加动态信息
        let path = "/tmp/data.txt";
        let _content = fs::read_to_string(path)
            .with_context(|| format!("无法读取文件: {}", path))?;
        
        // 2. 使用 ensure! 进行验证
        let value = 42;
        ensure!(value > 0, "值必须为正数，当前: {}", value);
        
        // 3. 使用 bail! 提前返回
        if value > 100 {
            bail!("值 {} 超出最大限制 100", value);
        }
        
        Ok(())
    }
    
    match good_error_handling_example() {
        Ok(_) => println!("  执行成功"),
        Err(e) => println!("  错误: {}", e),
    }
    println!();
    
    Ok(())
}

/*
=== 总结 ===

1. Anyhow 核心概念:

   类型:
   - anyhow::Result<T> = Result<T, anyhow::Error>
   - anyhow::Error = 统一错误类型
   
   宏:
   - bail!(msg) - 立即返回错误
   - ensure!(cond, msg) - 条件检查
   - anyhow!(msg) - 创建错误对象
   
   Trait:
   - Context - 添加错误上下文

2. 核心优势:

   ✓ 统一的错误类型
   ✓ 自动类型转换
   ✓ 丰富的上下文信息
   ✓ 错误链和回溯
   ✓ 简化的函数签名

3. 使用场景:

   适合:
   ✓ 应用程序开发
   ✓ CLI 工具
   ✓ 服务器应用
   ✓ 快速原型
   
   不适合:
   ✗ 库开发（用 thiserror）
   ✗ 需要精确错误匹配
   ✗ 公共 API

4. 最佳实践:

   DO:
   ✓ 总是添加 context
   ✓ 使用 with_context 添加动态信息
   ✓ 使用 ensure! 和 bail!
   ✓ 构建清晰的错误链
   
   DON'T:
   ✗ 在库代码中使用
   ✗ 忽略错误上下文
   ✗ 过度使用 unwrap()

5. Context vs with_context:

   context:
   - 立即求值
   - 适合静态字符串
   
   with_context:
   - 惰性求值（闭包）
   - 适合动态构建的消息
   - 仅在错误发生时求值

6. 错误链:

   多层 context 会形成错误链:
   Level 1: 应用初始化失败
   Level 2: 配置加载失败
   Level 3: 读取文件失败
   Level 4: No such file or directory (根本原因)

运行示例:
  cargo run --bin anyhow_detailed
*/
