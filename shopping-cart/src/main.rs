use shopping_cart::{Cart, Product};

fn main() {
    println!("=== 购物车系统示例 ===");
    
    let mut cart = Cart::new();
    
    // 创建商品
    let products = vec![
        Product::new(1, "MacBook Pro".to_string(), 12999.0),
        Product::new(2, "iPhone 15".to_string(), 5999.0),
        Product::new(3, "AirPods Pro".to_string(), 1999.0),
        Product::new(4, "Apple Watch".to_string(), 2999.0),
    ];
    
    // 场景1: 添加单个商品
    println!("\n--- 场景1: 添加商品 ---");
    cart.add_item(products[0].clone(), 1);
    cart.display();
    
    // 场景2: 添加更多商品（触发多件优惠）
    println!("\n--- 场景2: 添加更多商品 ---");
    cart.add_item(products[1].clone(), 1);
    cart.add_item(products[2].clone(), 2);
    cart.display();
    
    // 场景3: 移除商品
    println!("\n--- 场景3: 移除 iPhone ---");
    cart.remove_item(2);
    cart.display();
    
    // 场景4: 更新数量
    println!("\n--- 场景4: 更新 AirPods 数量为 1 ---");
    cart.update_quantity(3, 1);
    cart.display();
    
    // 场景5: 小额购买（无优惠）
    println!("\n--- 场景5: 新购物车，小额购买 ---");
    let mut small_cart = Cart::new();
    small_cart.add_item(products[2].clone(), 1);
    small_cart.display();
}
