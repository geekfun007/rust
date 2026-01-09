"""
PyO3 模块测试
"""
import pytest


def test_import():
    """测试模块导入"""
    import py_rust_module
    assert hasattr(py_rust_module, '__version__')
    assert py_rust_module.__version__ == '0.1.0'


class TestBasicFunctions:
    """基础函数测试"""
    
    def test_add(self):
        from py_rust_module import add
        assert add(1, 2) == 3
        assert add(-1, 1) == 0
        assert add(0, 0) == 0
    
    def test_factorial(self):
        from py_rust_module import factorial
        assert factorial(0) == 1
        assert factorial(1) == 1
        assert factorial(5) == 120
        assert factorial(10) == 3628800
        
        # 测试边界情况
        with pytest.raises(ValueError):
            factorial(21)  # 超过最大值
    
    def test_fibonacci(self):
        from py_rust_module import fibonacci
        assert fibonacci(0) == 0
        assert fibonacci(1) == 1
        assert fibonacci(2) == 1
        assert fibonacci(10) == 55
        assert fibonacci(20) == 6765
    
    def test_reverse_string(self):
        from py_rust_module import reverse_string
        assert reverse_string("hello") == "olleh"
        assert reverse_string("") == ""
        assert reverse_string("a") == "a"
        assert reverse_string("abcd") == "dcba"
    
    def test_is_palindrome(self):
        from py_rust_module import is_palindrome
        assert is_palindrome("racecar") is True
        assert is_palindrome("A man a plan a canal Panama") is True
        assert is_palindrome("hello") is False
        assert is_palindrome("") is True
    
    def test_word_frequency(self):
        from py_rust_module import word_frequency
        freq = word_frequency("hello world hello")
        assert freq["hello"] == 2
        assert freq["world"] == 1
    
    def test_quicksort(self):
        from py_rust_module import quicksort
        assert quicksort([3, 1, 4, 1, 5, 9, 2, 6]) == [1, 1, 2, 3, 4, 5, 6, 9]
        assert quicksort([]) == []
        assert quicksort([1]) == [1]
        assert quicksort([5, 4, 3, 2, 1]) == [1, 2, 3, 4, 5]
    
    def test_binary_search(self):
        from py_rust_module import binary_search
        arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        assert binary_search(arr, 5) == 4
        assert binary_search(arr, 1) == 0
        assert binary_search(arr, 10) == 9
        assert binary_search(arr, 11) is None
        assert binary_search(arr, 0) is None


class TestVector2D:
    """Vector2D 类测试"""
    
    def test_creation(self):
        from py_rust_module import Vector2D
        v = Vector2D(3.0, 4.0)
        assert v.x == 3.0
        assert v.y == 4.0
    
    def test_length(self):
        from py_rust_module import Vector2D
        v = Vector2D(3.0, 4.0)
        assert v.length() == 5.0
    
    def test_normalize(self):
        from py_rust_module import Vector2D
        v = Vector2D(3.0, 4.0)
        normalized = v.normalize()
        assert abs(normalized.length() - 1.0) < 1e-10
    
    def test_dot_product(self):
        from py_rust_module import Vector2D
        v1 = Vector2D(1.0, 0.0)
        v2 = Vector2D(0.0, 1.0)
        assert v1.dot(v2) == 0.0  # 垂直向量
        
        v3 = Vector2D(1.0, 1.0)
        v4 = Vector2D(1.0, 1.0)
        assert v3.dot(v4) == 2.0
    
    def test_arithmetic(self):
        from py_rust_module import Vector2D
        v1 = Vector2D(1.0, 2.0)
        v2 = Vector2D(3.0, 4.0)
        
        # 加法
        v_add = v1 + v2
        assert v_add.x == 4.0
        assert v_add.y == 6.0
        
        # 减法
        v_sub = v2 - v1
        assert v_sub.x == 2.0
        assert v_sub.y == 2.0
        
        # 标量乘法
        v_mul = v1 * 2.0
        assert v_mul.x == 2.0
        assert v_mul.y == 4.0
    
    def test_repr(self):
        from py_rust_module import Vector2D
        v = Vector2D(1.0, 2.0)
        assert repr(v) == "Vector2D(1, 2)"


class TestMatrix:
    """Matrix 类测试"""
    
    def test_creation(self):
        from py_rust_module import Matrix
        m = Matrix([[1.0, 2.0], [3.0, 4.0]])
        assert m.shape() == (2, 2)
    
    def test_zeros(self):
        from py_rust_module import Matrix
        m = Matrix.zeros(3, 4)
        assert m.shape() == (3, 4)
        assert m.get(0, 0) == 0.0
    
    def test_identity(self):
        from py_rust_module import Matrix
        m = Matrix.identity(3)
        assert m.shape() == (3, 3)
        assert m.get(0, 0) == 1.0
        assert m.get(0, 1) == 0.0
        assert m.get(1, 1) == 1.0
    
    def test_transpose(self):
        from py_rust_module import Matrix
        m = Matrix([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]])
        t = m.transpose()
        assert t.shape() == (3, 2)
        assert t.get(0, 0) == 1.0
        assert t.get(0, 1) == 4.0
    
    def test_matmul(self):
        from py_rust_module import Matrix
        m1 = Matrix([[1.0, 2.0], [3.0, 4.0]])
        m2 = Matrix([[5.0, 6.0], [7.0, 8.0]])
        result = m1.matmul(m2)
        assert result.shape() == (2, 2)
        assert result.get(0, 0) == 19.0  # 1*5 + 2*7
        assert result.get(0, 1) == 22.0  # 1*6 + 2*8


class TestUser:
    """User 类测试"""
    
    def test_creation(self):
        from py_rust_module import User
        user = User(1, "Alice", "alice@example.com", 25)
        assert user.id == 1
        assert user.name == "Alice"
        assert user.email == "alice@example.com"
        assert user.age == 25
    
    def test_json_serialization(self):
        from py_rust_module import User
        user = User(1, "Alice", "alice@example.com", 25)
        json_str = user.to_json()
        assert '"id":1' in json_str
        assert '"name":"Alice"' in json_str
        
        # 反序列化
        user2 = User.from_json(json_str)
        assert user2.id == user.id
        assert user2.name == user.name
    
    def test_to_dict(self):
        from py_rust_module import User
        user = User(1, "Alice", "alice@example.com", None)
        d = user.to_dict()
        assert d["id"] == 1
        assert d["name"] == "Alice"
        assert d["age"] is None


class TestStatistics:
    """Statistics 类测试"""
    
    def test_basic_stats(self):
        from py_rust_module import Statistics
        stats = Statistics([1.0, 2.0, 3.0, 4.0, 5.0])
        
        assert stats.mean() == 3.0
        assert stats.median() == 3.0
        assert stats.min() == 1.0
        assert stats.max() == 5.0
    
    def test_variance_and_std(self):
        from py_rust_module import Statistics
        stats = Statistics([2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0])
        
        # 验证方差计算
        assert abs(stats.variance() - 4.0) < 0.01
        assert abs(stats.std_dev() - 2.0) < 0.01
    
    def test_summary(self):
        from py_rust_module import Statistics
        stats = Statistics([1.0, 2.0, 3.0])
        summary = stats.summary()
        
        assert "mean" in summary
        assert "median" in summary
        assert "std_dev" in summary
        assert "count" in summary


class TestCounter:
    """Counter 类测试"""
    
    def test_creation(self):
        from py_rust_module import Counter
        c = Counter()
        assert len(c) == 0
    
    def test_from_iterable(self):
        from py_rust_module import Counter
        c = Counter.from_iterable(["a", "b", "a", "c", "a", "b"])
        assert c.get("a") == 3
        assert c.get("b") == 2
        assert c.get("c") == 1
        assert c.get("d") == 0
    
    def test_update(self):
        from py_rust_module import Counter
        c = Counter()
        c.update(["a", "a", "b"])
        assert c.get("a") == 2
        assert c.get("b") == 1
    
    def test_most_common(self):
        from py_rust_module import Counter
        c = Counter.from_iterable(["a", "b", "a", "c", "a", "b"])
        most = c.most_common(2)
        assert most[0] == ("a", 3)
        assert most[1] == ("b", 2)
    
    def test_total(self):
        from py_rust_module import Counter
        c = Counter.from_iterable(["a", "b", "a"])
        assert c.total() == 3


class TestJSON:
    """JSON 函数测试"""
    
    def test_parse_json(self):
        from py_rust_module import parse_json
        
        # 基本类型
        assert parse_json("null") is None
        assert parse_json("true") is True
        assert parse_json("42") == 42
        assert parse_json('"hello"') == "hello"
        
        # 数组
        arr = parse_json("[1, 2, 3]")
        assert arr == [1, 2, 3]
        
        # 对象
        obj = parse_json('{"name": "Alice", "age": 30}')
        assert obj["name"] == "Alice"
        assert obj["age"] == 30
    
    def test_to_json(self):
        from py_rust_module import to_json
        
        # 基本类型
        assert '"hello"' in to_json("hello")
        
        # 字典
        result = to_json({"name": "Bob", "age": 25})
        assert "Bob" in result
        assert "25" in result


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
