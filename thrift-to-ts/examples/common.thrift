// 通用类型定义

namespace js common
namespace py common

// 常量定义
const i32 MAX_PAGE_SIZE = 100
const string DEFAULT_LANGUAGE = "en"
const list<string> SUPPORTED_LANGUAGES = ["en", "zh", "ja", "ko"]

// 类型别名
typedef i64 Timestamp
typedef string UUID
typedef map<string, string> Metadata

// 通用错误码枚举
enum ErrorCode {
    OK = 0,
    UNKNOWN = 1,
    INVALID_ARGUMENT = 2,
    NOT_FOUND = 3,
    ALREADY_EXISTS = 4,
    PERMISSION_DENIED = 5,
    UNAUTHENTICATED = 6,
    INTERNAL = 7,
    UNAVAILABLE = 8,
    TIMEOUT = 9
}

// API 响应状态
struct ResponseStatus {
    1: required ErrorCode code
    2: optional string message
    3: optional Metadata details
}

// 通用分页请求
struct PageRequest {
    1: required i32 page = 1
    2: required i32 size = 20
    3: optional string sortBy
    4: optional bool ascending = true
}

// 通用分页响应
struct PageInfo {
    1: required i32 currentPage
    2: required i32 pageSize
    3: required i64 totalItems
    4: required i32 totalPages
    5: required bool hasNext
    6: required bool hasPrevious
}

// 键值对
struct KeyValue {
    1: required string key
    2: required string value
}

// 时间范围
struct TimeRange {
    1: optional Timestamp start
    2: optional Timestamp end
}

// 地理位置
struct GeoPoint {
    1: required double latitude
    2: required double longitude
}

// 货币金额
struct Money {
    1: required string currency
    2: required i64 amount  // 以分为单位
}

// 文件信息
struct FileInfo {
    1: required UUID id
    2: required string name
    3: required string mimeType
    4: required i64 size
    5: required string url
    6: required Timestamp createdAt
}

// 通用异常
exception ServiceException {
    1: required ErrorCode code
    2: required string message
    3: optional Metadata details
}
