// Rust 详解与实战教程 - 主程序

use std::io::{self, Write};

// 声明所有模块
mod basics;
mod types;
mod ownership;
mod concurrency;
#[path = "io/mod.rs"]
mod io_ops;
mod network;
mod best_practices;

fn main() {
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║                                                          ║");
    println!("║           Rust 详解与实战教程                            ║");
    println!("║         Comprehensive Rust Tutorial                     ║");
    println!("║                                                          ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    
    loop {
        print_menu();
        
        let choice = get_user_input();
        
        match choice.trim() {
            "0" => {
                println!("\n感谢使用！再见！");
                break;
            }
            "1" => run_basics_demos(),
            "2" => run_types_demos(),
            "3" => run_ownership_demos(),
            "4" => run_concurrency_demos(),
            "5" => run_io_demos(),
            "6" => run_network_demos(),
            "7" => run_best_practices(),
            "8" => run_all_demos(),
            _ => println!("\n⚠️  无效选择，请重新输入！"),
        }
        
        println!("\n按 Enter 继续...");
        let _ = get_user_input();
    }
}

fn print_menu() {
    println!("\n");
    println!("┌────────────────────────────────────────────────────────┐");
    println!("│                     主菜单                              │");
    println!("├────────────────────────────────────────────────────────┤");
    println!("│  1. 基础语法 (Syntax Basics)                           │");
    println!("│  2. 数据类型 (Data Types)                              │");
    println!("│  3. 所有权系统 (Ownership System)                      │");
    println!("│  4. 并发编程 (Concurrency)                             │");
    println!("│  5. I/O 操作 (I/O Operations)                          │");
    println!("│  6. 网络编程 (Network Programming)                     │");
    println!("│  7. 最佳实践 (Best Practices)                          │");
    println!("│  8. 运行所有示例 (Run All Demos)                       │");
    println!("│  0. 退出 (Exit)                                        │");
    println!("└────────────────────────────────────────────────────────┘");
    print!("\n请选择 (0-8): ");
    io::stdout().flush().unwrap();
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");
    input
}

fn run_basics_demos() {
    println!("\n┌────────────────────────────────────────────────────────┐");
    println!("│                  基础语法示例                           │");
    println!("└────────────────────────────────────────────────────────┘");
    
    basics::syntax::run_all();
    basics::control_flow::run_all();
    basics::functions::run_all();
    basics::data_types::run_all();
}

fn run_types_demos() {
    println!("\n┌────────────────────────────────────────────────────────┐");
    println!("│                  数据类型示例                           │");
    println!("└────────────────────────────────────────────────────────┘");
    
    types::primitives::run_all();
    types::strings::run_all();
    types::collections::run_all();
    types::structs::run_all();
    types::enums::run_all();
    types::datetime::run_all();
    types::regex_examples::run_all();
    types::errors::run_all();
}

fn run_ownership_demos() {
    println!("\n┌────────────────────────────────────────────────────────┐");
    println!("│                 所有权系统示例                          │");
    println!("└────────────────────────────────────────────────────────┘");
    
    ownership::ownership::run_all();
    ownership::borrowing::run_all();
    ownership::slices::run_all();
    ownership::lifetimes::run_all();
}

fn run_concurrency_demos() {
    println!("\n┌────────────────────────────────────────────────────────┐");
    println!("│                  并发编程示例                           │");
    println!("└────────────────────────────────────────────────────────┘");
    
    concurrency::threads::run_all();
    
    // 异步示例需要 tokio 运行时
    println!("\n异步编程示例需要 tokio 运行时，请使用:");
    println!("  cargo run --example async_demo");
    
    concurrency::channels::run_all();
}

fn run_io_demos() {
    println!("\n┌────────────────────────────────────────────────────────┐");
    println!("│                   I/O 操作示例                          │");
    println!("└────────────────────────────────────────────────────────┘");
    
    io_ops::filesystem::run_all();
    io_ops::files::run_all();
}

fn run_network_demos() {
    println!("\n┌────────────────────────────────────────────────────────┐");
    println!("│                  网络编程示例                           │");
    println!("└────────────────────────────────────────────────────────┘");
    
    network::tcp_server::run_all();
    network::tcp_client::run_all();
    network::http_server::run_all();
    network::http_client::run_all();
    
    println!("\n注意：网络示例主要展示代码结构。");
    println!("实际运行的网络服务器和客户端请参考 examples/ 目录。");
}

fn run_best_practices() {
    println!("\n┌────────────────────────────────────────────────────────┐");
    println!("│                   最佳实践指南                          │");
    println!("└────────────────────────────────────────────────────────┘");
    
    best_practices::guidelines::run_all();
}

fn run_all_demos() {
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║              运行所有示例 (这可能需要一些时间)             ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    
    run_basics_demos();
    run_types_demos();
    run_ownership_demos();
    run_concurrency_demos();
    run_io_demos();
    run_network_demos();
    run_best_practices();
    
    println!("\n╔══════════════════════════════════════════════════════════╗");
    println!("║                    所有示例运行完毕！                      ║");
    println!("╚══════════════════════════════════════════════════════════╝");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main_compiles() {
        // 确保主程序能够编译
        assert!(true);
    }
}
