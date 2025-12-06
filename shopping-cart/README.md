# 购物车系统 - Rust 模块化示例

展示电商购物车系统的模块化设计。

## 项目结构

```
shopping-cart/
├── Cargo.toml
└── src/
    ├── main.rs             # 主程序
    ├── lib.rs              # 库入口
    ├── cart/               # 购物车模块
    │   ├── mod.rs
    │   ├── cart.rs         # 购物车逻辑
    │   └── item.rs         # 购物车项
    ├── product/            # 商品模块
    │   └── mod.rs
    └── discount/           # 折扣模块
        └── mod.rs
```

## 功能特性

- ✅ 添加/删除商品
- ✅ 更新商品数量
- ✅ 自动计算小计和总价
- ✅ 满减优惠（满100打9折）
- ✅ 多件优惠（3件及以上额外95折）
- ✅ 友好的显示界面

## 运行示例

```bash
cargo run
```

## 运行测试

```bash
cargo test
```

## 代码示例

```rust
use shopping_cart::{Cart, Product};

let mut cart = Cart::new();
let product = Product::new(1, "商品A".to_string(), 99.0);

cart.add_item(product, 2);
cart.display();
```

## 优惠规则

1. **满减优惠**: 满100元打9折
2. **多件优惠**: 购买3件及以上商品额外95折
3. **叠加优惠**: 两种优惠可以叠加使用

## 学习要点

- 模块化设计（cart, product, discount）
- 业务逻辑封装
- 测试驱动开发
- 链式优惠计算
