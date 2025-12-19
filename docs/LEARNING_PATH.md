# Rust 核心概念学习路径

## 学习顺序建议

```
第一阶段：基础概念
  1. 所有权基础
     └─> 运行: cargo run --example memory_layout
  
  2. 借用入门
     └─> 运行: cargo run --example borrow_checker
  
  3. 切片使用
     └─> 查看: src/ownership/slices.rs

第二阶段：深入理解
  4. 生命周期详解
     └─> 运行: cargo run --example lifetime_deep_dive
  
  5. 类型系统
     └─> 运行: cargo run --example type_annotations
  
  6. 内存布局
     └─> 查看: docs/CORE_PRINCIPLES.md

第三阶段：高级特性
  7. 智能指针 (Box, Rc, Arc)
  8. 并发编程
     └─> 查看: src/concurrency/
  9. 异步编程
     └─> 查看: src/concurrency/async_await.rs

第四阶段：实战应用
  10. 错误处理
      └─> 查看: src/types/errors.rs
  11. I/O 操作
      └─> 查看: src/io/
  12. 网络编程
      └─> 查看: src/network/
