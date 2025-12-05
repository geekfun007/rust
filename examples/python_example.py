#!/usr/bin/env python3
"""
Python 使用 Rust 包的示例
"""

import rust_py_example as rpe


def main():
    print("=== Python 使用 Rust 包示例 ===\n")

    # 1. 用户管理示例
    print("1. 用户管理:")
    user1 = rpe.PyUser("张三", 25, "zhangsan@example.com")
    user2 = rpe.PyUser("李四", 16, "lisi@example.com")

    print(f"  {user1.description()}")
    print(f"  是否成年: {user1.is_adult()}")
    print(f"  {user2.description()}")
    print(f"  是否成年: {user2.is_adult()}")
    print(f"  用户字典: {user1.to_dict()}\n")

    # 2. 数学计算示例
    print("2. 数学计算:")
    print(f"  斐波那契数列 (n=10): {rpe.fibonacci(10)}")
    print(f"  阶乘 (5!): {rpe.factorial(5)}")
    print(f"  17 是质数吗? {rpe.is_prime(17)}")
    print(f"  18 是质数吗? {rpe.is_prime(18)}\n")

    # 3. 字符串处理示例
    print("3. 字符串处理:")
    text = "hello world from rust"
    print(f"  原始文本: {text}")
    print(f"  反转: {rpe.reverse_string(text)}")
    print(f"  单词数: {rpe.word_count(text)}")
    print(f"  首字母大写: {rpe.capitalize_words(text)}\n")

    # 4. 批量处理示例
    print("4. 批量处理:")
    numbers = [0, 1, 2, 3, 4, 5, 10, 15, 20]
    print(f"  输入数字: {numbers}")
    print(f"  斐波那契批量计算: {rpe.fibonacci_batch(numbers)}")

    test_primes = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 20]
    print(f"  输入数字: {test_primes}")
    print(f"  过滤质数: {rpe.filter_primes(test_primes)}\n")

    # 5. 性能对比示例
    print("5. 性能测试:")
    import time

    # Python 实现的斐波那契
    def fib_python(n):
        if n <= 1:
            return n
        a, b = 0, 1
        for _ in range(2, n + 1):
            a, b = b, a + b
        return b

    n = 30
    # Rust 版本
    start = time.time()
    for _ in range(10000):
        _ = rpe.fibonacci(n)
    rust_time = time.time() - start

    # Python 版本
    start = time.time()
    for _ in range(10000):
        _ = fib_python(n)
    python_time = time.time() - start

    print(f"  计算 fibonacci({n}) 10000 次:")
    print(f"  Rust 版本耗时: {rust_time:.4f} 秒")
    print(f"  Python 版本耗时: {python_time:.4f} 秒")
    print(f"  性能提升: {python_time / rust_time:.2f}x")


if __name__ == "__main__":
    main()
