"""
Rust-Python Example Package

这是一个展示如何在 Python 中使用 Rust 包的示例项目。
"""

from ._rust_py_example import (
    PyUser,
    fibonacci,
    is_prime,
    factorial,
    reverse_string,
    word_count,
    capitalize_words,
    fibonacci_batch,
    filter_primes,
)

__version__ = "0.1.0"

__all__ = [
    "PyUser",
    "fibonacci",
    "is_prime",
    "factorial",
    "reverse_string",
    "word_count",
    "capitalize_words",
    "fibonacci_batch",
    "filter_primes",
]
