#!/bin/bash

# Rust 示例快速测试脚本

echo "🦀 Rust 核心概念示例测试"
echo "========================="
echo ""

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 1. 类型转换
echo -e "${YELLOW}1. 测试类型转换 (conversions)${NC}"
cargo run --bin conversions
echo ""

# 2. 错误处理
echo -e "${YELLOW}2. 测试错误处理 (error_handling)${NC}"
cargo run --bin error_handling
echo ""

# 3. 文件操作
echo -e "${YELLOW}3. 测试文件操作 (file_operations)${NC}"
cargo run --bin file_operations
echo ""

# 4. HTTP 客户端（需要网络）
echo -e "${YELLOW}4. 测试 HTTP 客户端 (http_client)${NC}"
echo "注意: 需要网络连接"
cargo run --bin http_client
echo ""

# 5. HTTP 服务器（后台运行）
echo -e "${YELLOW}5. HTTP 服务器 (http_server)${NC}"
echo "提示: 运行以下命令启动服务器:"
echo "  cargo run --bin http_server"
echo "然后访问 http://localhost:3000"
echo ""

echo -e "${GREEN}✅ 测试完成！${NC}"
