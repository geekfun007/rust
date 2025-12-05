#!/usr/bin/env python3
"""
性能基准测试脚本

比较 Rust 实现和纯 Python 实现的性能差异
"""

import time
import statistics
from typing import Callable, List, Tuple


def benchmark(func: Callable, args: tuple, iterations: int = 1000) -> Tuple[float, float]:
    """
    运行性能基准测试
    
    返回: (平均时间, 标准差)
    """
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        func(*args)
        end = time.perf_counter()
        times.append(end - start)
    
    return statistics.mean(times), statistics.stdev(times)


# Python 实现
def fibonacci_python(n: int) -> int:
    """纯 Python 实现的斐波那契"""
    if n <= 1:
        return n
    a, b = 0, 1
    for _ in range(2, n + 1):
        a, b = b, a + b
    return b


def is_prime_python(n: int) -> bool:
    """纯 Python 实现的质数判断"""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    for i in range(3, int(n**0.5) + 1, 2):
        if n % i == 0:
            return False
    return True


def factorial_python(n: int) -> int:
    """纯 Python 实现的阶乘"""
    if n <= 1:
        return 1
    result = 1
    for i in range(2, n + 1):
        result *= i
    return result


def reverse_string_python(s: str) -> str:
    """纯 Python 实现的字符串反转"""
    return s[::-1]


def main():
    print("=" * 70)
    print("Rust vs Python 性能基准测试")
    print("=" * 70)
    
    try:
        import rust_py_example as rpe
        rust_available = True
    except ImportError:
        print("\n⚠️  无法导入 rust_py_example，请先运行: maturin develop --release")
        rust_available = False
        return
    
    iterations = 10000
    print(f"\n每个测试运行 {iterations} 次迭代\n")
    
    # 测试用例
    test_cases = [
        {
            "name": "斐波那契数列 (n=30)",
            "rust_func": rpe.fibonacci,
            "python_func": fibonacci_python,
            "args": (30,),
        },
        {
            "name": "质数判断 (n=1000000007)",
            "rust_func": rpe.is_prime,
            "python_func": is_prime_python,
            "args": (1000000007,),
        },
        {
            "name": "阶乘 (n=20)",
            "rust_func": rpe.factorial,
            "python_func": factorial_python,
            "args": (20,),
        },
        {
            "name": "字符串反转 (长度=100)",
            "rust_func": rpe.reverse_string,
            "python_func": reverse_string_python,
            "args": ("a" * 100,),
        },
    ]
    
    results = []
    
    for test in test_cases:
        print(f"测试: {test['name']}")
        print("-" * 70)
        
        # Rust 实现
        rust_mean, rust_std = benchmark(test["rust_func"], test["args"], iterations)
        print(f"  Rust 实现:")
        print(f"    平均时间: {rust_mean * 1000000:.2f} μs")
        print(f"    标准差:   {rust_std * 1000000:.2f} μs")
        
        # Python 实现
        python_mean, python_std = benchmark(test["python_func"], test["args"], iterations)
        print(f"  Python 实现:")
        print(f"    平均时间: {python_mean * 1000000:.2f} μs")
        print(f"    标准差:   {python_std * 1000000:.2f} μs")
        
        # 性能提升
        speedup = python_mean / rust_mean
        print(f"  性能提升: {speedup:.2f}x 🚀")
        print()
        
        results.append({
            "name": test["name"],
            "rust_time": rust_mean,
            "python_time": python_mean,
            "speedup": speedup,
        })
    
    # 总结
    print("=" * 70)
    print("总结")
    print("=" * 70)
    
    avg_speedup = statistics.mean([r["speedup"] for r in results])
    print(f"\n平均性能提升: {avg_speedup:.2f}x")
    
    print("\n各项测试结果:")
    for r in results:
        bar_length = int(r["speedup"] * 5)
        bar = "█" * min(bar_length, 50)
        print(f"  {r['name']:40s} {r['speedup']:6.2f}x {bar}")
    
    # 批量处理测试
    print("\n" + "=" * 70)
    print("批量处理性能测试")
    print("=" * 70)
    
    numbers = list(range(100))
    
    # Rust 批量处理
    start = time.time()
    for _ in range(100):
        _ = rpe.fibonacci_batch(numbers)
    rust_batch_time = time.time() - start
    
    # Python 批量处理
    start = time.time()
    for _ in range(100):
        _ = [fibonacci_python(n) for n in numbers]
    python_batch_time = time.time() - start
    
    batch_speedup = python_batch_time / rust_batch_time
    print(f"\n批量计算 fibonacci(0-99) 100 次:")
    print(f"  Rust:   {rust_batch_time:.4f} 秒")
    print(f"  Python: {python_batch_time:.4f} 秒")
    print(f"  性能提升: {batch_speedup:.2f}x 🚀")
    
    print("\n" + "=" * 70)
    print("✅ 基准测试完成")
    print("=" * 70)


if __name__ == "__main__":
    main()
