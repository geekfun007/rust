#!/bin/bash

echo "=== 测试所有 Rust 教程示例 ==="
echo ""

BINS=(
    "conversions"
    "error_handling"
    "http_client"
    "file_operations"
    "arc_vs_mutex"
    "derive_and_string"
    "impl_detailed"
    "ok_methods_comparison"
    "trait_detailed"
    "map_detailed"
    "derive_macros_detailed"
    "anyhow_detailed"
)

SUCCESS=0
FAILED=0

for bin in "${BINS[@]}"; do
    echo "测试: $bin"
    if cargo run --bin "$bin" > /dev/null 2>&1; then
        echo "  ✅ 通过"
        ((SUCCESS++))
    else
        echo "  ❌ 失败"
        ((FAILED++))
    fi
done

echo ""
echo "=== 测试结果 ==="
echo "成功: $SUCCESS"
echo "失败: $FAILED"
