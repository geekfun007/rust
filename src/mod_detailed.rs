// Rust 模块化管理详解
//
// 本教程涵盖 Rust 模块系统的所有核心概念
// 包括 mod, pub, use, crate, super, self 等

fn main() {
    println!("=== Rust 模块化管理详解 ===\n");
    
    // 1. 模块基础
    demo_module_basics();
    
    // 2. 可见性（pub）
    demo_visibility();
    
    // 3. use 导入
    demo_use_imports();
    
    // 4. 路径（Path）
    demo_paths();
    
    // 5. 文件组织
    demo_file_organization();
    
    // 6. 模块实战
    demo_real_world_examples();
    
    println!("\n✅ 所有示例运行成功！");
}

// ============================================
// 1. 模块基础
// ============================================
fn demo_module_basics() {
    println!("--- 1. 模块基础 ---\n");
    
    println!("什么是模块？");
    println!("  - 组织代码的基本单位");
    println!("  - 控制可见性和私有性");
    println!("  - 形成层次结构");
    println!("  - 避免命名冲突\n");
    
    println!("关键字:");
    println!("  📌 mod   - 定义模块");
    println!("  📌 pub   - 公开可见");
    println!("  📌 use   - 导入路径");
    println!("  📌 crate - 当前 crate 根");
    println!("  📌 super - 父模块");
    println!("  📌 self  - 当前模块\n");
    
    // 基础模块定义
    println!("基础示例:");
    
    mod basic {
        pub fn public_function() {
            println!("  这是公开函数");
        }
        
        fn private_function() {
            println!("  这是私有函数");
        }
        
        pub fn call_private() {
            println!("  调用私有函数:");
            private_function();
        }
    }
    
    basic::public_function();
    basic::call_private();
    // basic::private_function(); // 错误！私有函数不可访问
    println!();
    
    // 嵌套模块
    println!("嵌套模块:");
    
    mod outer {
        pub mod inner {
            pub fn function() {
                println!("  内部模块的函数");
            }
        }
    }
    
    outer::inner::function();
    println!();
}

// ============================================
// 2. 可见性（pub）
// ============================================
fn demo_visibility() {
    println!("--- 2. 可见性（pub）---\n");
    
    println!("可见性级别:");
    println!("  🔒 默认     - 私有（仅当前模块）");
    println!("  🔓 pub      - 公开（所有人可见）");
    println!("  📦 pub(crate)   - crate 内可见");
    println!("  📁 pub(super)   - 父模块可见");
    println!("  🗂️  pub(in path) - 指定路径可见\n");
    
    mod visibility_demo {
        // 默认私有
        fn private() {
            println!("  私有函数");
        }
        
        // 公开
        pub fn public() {
            println!("  公开函数");
            private(); // 同模块可访问
        }
        
        // crate 内公开
        pub(crate) fn crate_visible() {
            println!("  crate 内可见");
        }
        
        // 父模块可见
        pub(super) fn super_visible() {
            println!("  父模块可见");
        }
        
        // 指定路径可见（示例）
        // pub(in crate::some_module) fn path_visible() {
        //     println!("  指定路径可见");
        // }
        
        pub fn path_visible() {
            println!("  指定路径可见（简化示例）");
        }
        
        pub mod nested {
            pub fn test() {
                // 可以访问父模块的 pub(super)
                super::super_visible();
            }
        }
    }
    
    println!("访问测试:");
    visibility_demo::public();
    visibility_demo::crate_visible();
    visibility_demo::super_visible();
    visibility_demo::path_visible();
    visibility_demo::nested::test();
    println!();
    
    // 结构体可见性
    println!("结构体可见性:");
    
    mod struct_demo {
        pub struct PublicStruct {
            pub public_field: i32,
            private_field: i32,
        }
        
        impl PublicStruct {
            pub fn new(value: i32) -> Self {
                PublicStruct {
                    public_field: value,
                    private_field: value * 2,
                }
            }
            
            pub fn get_private(&self) -> i32 {
                self.private_field
            }
        }
    }
    
    let s = struct_demo::PublicStruct::new(10);
    println!("  公开字段: {}", s.public_field);
    println!("  私有字段（通过方法）: {}", s.get_private());
    println!();
    
    // 枚举可见性
    println!("枚举可见性:");
    
    mod enum_demo {
        pub enum Status {
            Active,
            Inactive,
            Pending,
        }
    }
    
    let status = enum_demo::Status::Active;
    println!("  状态: {:?}", status as i32);
    println!();
}

// ============================================
// 3. use 导入
// ============================================
fn demo_use_imports() {
    println!("--- 3. use 导入 ---\n");
    
    println!("use 的作用:");
    println!("  - 简化路径");
    println!("  - 避免重复");
    println!("  - 提高可读性\n");
    
    // 基础 use
    println!("基础 use:");
    
    mod my_module {
        pub fn function_a() {
            println!("  函数 A");
        }
        
        pub fn function_b() {
            println!("  函数 B");
        }
    }
    
    use my_module::function_a;
    function_a();
    println!();
    
    // 嵌套 use
    println!("嵌套 use:");
    
    mod parent {
        pub mod child {
            pub fn func1() {
                println!("  子函数 1");
            }
            pub fn func2() {
                println!("  子函数 2");
            }
        }
    }
    
    use parent::child::{func1, func2};
    func1();
    func2();
    println!();
    
    // as 重命名
    println!("as 重命名:");
    
    mod module_a {
        pub fn process() {
            println!("  模块 A 处理");
        }
    }
    
    mod module_b {
        pub fn process() {
            println!("  模块 B 处理");
        }
    }
    
    use module_a::process as process_a;
    use module_b::process as process_b;
    
    process_a();
    process_b();
    println!();
    
    // glob import
    println!("Glob 导入（*）:");
    
    mod utils {
        pub fn util_a() { println!("  工具 A"); }
        pub fn util_b() { println!("  工具 B"); }
        pub fn util_c() { println!("  工具 C"); }
    }
    
    use utils::*;
    util_a();
    util_b();
    util_c();
    println!();
    
    println!("注意: glob 导入可能导致命名冲突，谨慎使用");
    println!();
}

// ============================================
// 4. 路径（Path）
// ============================================
fn demo_paths() {
    println!("--- 4. 路径（Path）---\n");
    
    println!("路径类型:");
    println!("  📍 绝对路径 - 从 crate 根开始");
    println!("  📍 相对路径 - 从当前模块开始\n");
    
    println!("路径关键字:");
    println!("  crate - crate 根");
    println!("  super - 父模块");
    println!("  self  - 当前模块\n");
    
    mod path_demo {
        pub fn current_module() {
            println!("  当前模块函数");
        }
        
        pub mod child {
            pub fn child_function() {
                println!("  子模块函数");
                
                // 绝对路径（示例 - 在实际文件中使用）
                // crate::my_module::function();
                println!("  (绝对路径: crate::module::function)");
                
                // 相对路径 - super
                super::current_module();
                
                // 相对路径 - self
                self::sibling_function();
            }
            
            pub fn sibling_function() {
                println!("  兄弟函数");
            }
        }
    }
    
    println!("路径示例:");
    path_demo::child::child_function();
    println!();
    
    // 路径选择建议
    println!("路径选择建议:");
    println!("  ✓ 优先使用绝对路径（更稳定）");
    println!("  ✓ super 用于访问父模块");
    println!("  ✓ self 用于显式当前模块");
    println!();
}

// ============================================
// 5. 文件组织
// ============================================
fn demo_file_organization() {
    println!("--- 5. 文件组织 ---\n");
    
    println!("文件组织模式:\n");
    
    println!("模式 1: 单文件模块");
    println!("  项目结构:");
    println!("    src/");
    println!("    ├── main.rs");
    println!("    └── my_module.rs");
    println!();
    println!("  在 main.rs 中:");
    println!("    mod my_module;  // 自动查找 my_module.rs");
    println!();
    
    println!("模式 2: 目录模块");
    println!("  项目结构:");
    println!("    src/");
    println!("    ├── main.rs");
    println!("    └── my_module/");
    println!("        ├── mod.rs     // 模块根");
    println!("        ├── sub1.rs");
    println!("        └── sub2.rs");
    println!();
    println!("  在 main.rs 中:");
    println!("    mod my_module;  // 查找 my_module/mod.rs");
    println!();
    println!("  在 my_module/mod.rs 中:");
    println!("    mod sub1;");
    println!("    mod sub2;");
    println!();
    
    println!("模式 3: 新式目录模块（Rust 2018+）");
    println!("  项目结构:");
    println!("    src/");
    println!("    ├── main.rs");
    println!("    ├── my_module.rs   // 模块根");
    println!("    └── my_module/");
    println!("        ├── sub1.rs");
    println!("        └── sub2.rs");
    println!();
    println!("  在 main.rs 中:");
    println!("    mod my_module;");
    println!();
    println!("  在 my_module.rs 中:");
    println!("    mod sub1;");
    println!("    mod sub2;");
    println!();
    
    println!("最佳实践:");
    println!("  ✓ 小模块用单文件");
    println!("  ✓ 大模块用目录");
    println!("  ✓ 优先使用新式组织（Rust 2018+）");
    println!("  ✓ 保持模块层次清晰");
    println!();
}

// ============================================
// 6. 模块实战
// ============================================
fn demo_real_world_examples() {
    println!("--- 6. 模块实战 ---\n");
    
    println!("实战案例: Web 应用模块结构\n");
    web_app_example();
    
    println!("\n实战案例: 库模块结构\n");
    library_example();
    
    println!("\n实战案例: 游戏引擎模块\n");
    game_engine_example();
}

fn web_app_example() {
    println!("Web 应用典型结构:");
    println!("  src/");
    println!("  ├── main.rs          // 入口");
    println!("  ├── config.rs        // 配置");
    println!("  ├── models/          // 数据模型");
    println!("  │   ├── mod.rs");
    println!("  │   ├── user.rs");
    println!("  │   └── post.rs");
    println!("  ├── handlers/        // 请求处理");
    println!("  │   ├── mod.rs");
    println!("  │   ├── user.rs");
    println!("  │   └── post.rs");
    println!("  ├── services/        // 业务逻辑");
    println!("  │   ├── mod.rs");
    println!("  │   ├── auth.rs");
    println!("  │   └── database.rs");
    println!("  └── utils/           // 工具函数");
    println!("      ├── mod.rs");
    println!("      └── validation.rs");
    println!();
    
    println!("main.rs 示例:");
    println!("  mod config;");
    println!("  mod models;");
    println!("  mod handlers;");
    println!("  mod services;");
    println!("  mod utils;");
    println!();
    println!("  use config::Config;");
    println!("  use handlers::user as user_handler;");
    println!();
    
    // 模拟模块
    mod web_demo {
        pub mod models {
            pub struct User {
                pub id: u32,
                pub name: String,
            }
            
            impl User {
                pub fn new(id: u32, name: String) -> Self {
                    User { id, name }
                }
            }
        }
        
        pub mod handlers {
            use super::models::User;
            
            pub fn get_user(id: u32) -> User {
                User::new(id, "Alice".to_string())
            }
        }
        
        pub mod services {
            pub fn authenticate(_token: &str) -> bool {
                true
            }
        }
    }
    
    use web_demo::handlers;
    use web_demo::services;
    
    println!("运行示例:");
    if services::authenticate("token") {
        let user = handlers::get_user(1);
        println!("  获取用户: {} (ID: {})", user.name, user.id);
    }
}

fn library_example() {
    println!("库模块典型结构:");
    println!("  src/");
    println!("  ├── lib.rs           // 库根");
    println!("  ├── error.rs         // 错误类型");
    println!("  ├── core/            // 核心功能");
    println!("  │   ├── mod.rs");
    println!("  │   ├── parser.rs");
    println!("  │   └── compiler.rs");
    println!("  ├── utils/           // 工具");
    println!("  │   ├── mod.rs");
    println!("  │   └── helper.rs");
    println!("  └── prelude.rs       // 预导入");
    println!();
    
    println!("lib.rs 示例:");
    println!("  pub mod error;");
    println!("  pub mod core;");
    println!("  pub mod utils;");
    println!("  pub mod prelude;");
    println!();
    println!("  // 重导出常用类型");
    println!("  pub use error::{{Error, Result}};");
    println!("  pub use core::parser::Parser;");
    println!();
    
    println!("prelude.rs 示例:");
    println!("  pub use crate::error::{{Error, Result}};");
    println!("  pub use crate::core::{{Parser, Compiler}};");
    println!();
    
    println!("用户使用:");
    println!("  use mylib::prelude::*;  // 导入所有常用类型");
}

fn game_engine_example() {
    println!("游戏引擎典型结构:");
    println!("  src/");
    println!("  ├── lib.rs           // 引擎根");
    println!("  ├── core/            // 核心系统");
    println!("  │   ├── mod.rs");
    println!("  │   ├── engine.rs");
    println!("  │   └── loop.rs");
    println!("  ├── graphics/        // 图形");
    println!("  │   ├── mod.rs");
    println!("  │   ├── renderer.rs");
    println!("  │   └── shader.rs");
    println!("  ├── physics/         // 物理");
    println!("  │   ├── mod.rs");
    println!("  │   └── collision.rs");
    println!("  ├── audio/           // 音频");
    println!("  │   └── mod.rs");
    println!("  └── ecs/             // 实体组件系统");
    println!("      ├── mod.rs");
    println!("      ├── entity.rs");
    println!("      └── component.rs");
    println!();
    
    // 模拟结构
    mod game_demo {
        pub mod core {
            pub struct Engine;
            impl Engine {
                pub fn new() -> Self { Engine }
                pub fn run(&self) {
                    println!("  引擎运行中...");
                }
            }
        }
        
        pub mod graphics {
            pub struct Renderer;
            impl Renderer {
                pub fn new() -> Self { Renderer }
                pub fn render(&self) {
                    println!("  渲染帧");
                }
            }
        }
        
        pub mod physics {
            pub fn update() {
                println!("  更新物理");
            }
        }
    }
    
    use game_demo::core::Engine;
    use game_demo::graphics::Renderer;
    use game_demo::physics;
    
    println!("运行示例:");
    let engine = Engine::new();
    let renderer = Renderer::new();
    
    engine.run();
    renderer.render();
    physics::update();
}

/*
=== 总结 ===

1. 模块系统核心:

   关键字:
   - mod   - 定义模块
   - pub   - 公开可见性
   - use   - 导入
   - crate - crate 根
   - super - 父模块
   - self  - 当前模块

2. 可见性级别:

   🔒 默认           - 私有
   🔓 pub            - 公开
   📦 pub(crate)     - crate 内
   📁 pub(super)     - 父模块
   🗂️  pub(in path)  - 指定路径

3. 文件组织:

   小模块:
   - 单文件 (my_module.rs)
   
   大模块:
   - 目录 + mod.rs（旧式）
   - my_module.rs + my_module/（新式）

4. 最佳实践:

   DO:
   ✓ 清晰的模块层次
   ✓ 合理的可见性控制
   ✓ 使用 use 简化路径
   ✓ 重导出常用类型
   
   DON'T:
   ✗ 过深的嵌套
   ✗ 过度使用 pub
   ✗ glob 导入（use *）
   ✗ 循环依赖

5. 典型结构:

   应用程序:
   - models/    - 数据模型
   - handlers/  - 处理逻辑
   - services/  - 业务逻辑
   - utils/     - 工具函数
   
   库:
   - core/      - 核心功能
   - error.rs   - 错误类型
   - prelude.rs - 预导入
   - lib.rs     - 重导出

运行示例:
  cargo run --bin mod_detailed
*/
