#!/usr/bin/env python3
"""
Python 测试套件
"""

import pytest
import rust_py_example as rpe


class TestUser:
    """用户类测试"""

    def test_create_user(self):
        user = rpe.PyUser("Alice", 25, "alice@example.com")
        assert user.name == "Alice"
        assert user.age == 25
        assert user.email == "alice@example.com"

    def test_user_description(self):
        user = rpe.PyUser("Bob", 30, "bob@example.com")
        desc = user.description()
        assert "Bob" in desc
        assert "30" in desc
        assert "bob@example.com" in desc

    def test_is_adult(self):
        adult = rpe.PyUser("Adult", 18, "adult@example.com")
        child = rpe.PyUser("Child", 17, "child@example.com")
        assert adult.is_adult() is True
        assert child.is_adult() is False

    def test_to_dict(self):
        user = rpe.PyUser("Charlie", 28, "charlie@example.com")
        user_dict = user.to_dict()
        assert user_dict["name"] == "Charlie"
        assert user_dict["age"] == 28
        assert user_dict["email"] == "charlie@example.com"


class TestMath:
    """数学函数测试"""

    def test_fibonacci(self):
        assert rpe.fibonacci(0) == 0
        assert rpe.fibonacci(1) == 1
        assert rpe.fibonacci(2) == 1
        assert rpe.fibonacci(10) == 55
        assert rpe.fibonacci(20) == 6765

    def test_is_prime(self):
        assert rpe.is_prime(2) is True
        assert rpe.is_prime(3) is True
        assert rpe.is_prime(4) is False
        assert rpe.is_prime(17) is True
        assert rpe.is_prime(18) is False
        assert rpe.is_prime(97) is True

    def test_factorial(self):
        assert rpe.factorial(0) == 1
        assert rpe.factorial(1) == 1
        assert rpe.factorial(5) == 120
        assert rpe.factorial(10) == 3628800


class TestText:
    """字符串处理测试"""

    def test_reverse_string(self):
        assert rpe.reverse_string("hello") == "olleh"
        assert rpe.reverse_string("rust") == "tsur"
        assert rpe.reverse_string("") == ""

    def test_word_count(self):
        assert rpe.word_count("hello world") == 2
        assert rpe.word_count("one two three four") == 4
        assert rpe.word_count("") == 0
        assert rpe.word_count("   spaces   ") == 1

    def test_capitalize_words(self):
        assert rpe.capitalize_words("hello world") == "Hello World"
        assert rpe.capitalize_words("rust programming") == "Rust Programming"


class TestBatch:
    """批量处理测试"""

    def test_fibonacci_batch(self):
        numbers = [0, 1, 2, 5, 10]
        results = rpe.fibonacci_batch(numbers)
        assert results == [0, 1, 1, 5, 55]

    def test_filter_primes(self):
        numbers = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        primes = rpe.filter_primes(numbers)
        assert primes == [2, 3, 5, 7, 11]


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
