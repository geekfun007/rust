// Common types shared across services
namespace go example.common

// Pagination request
struct PageRequest {
    1: required i32 page = 1
    2: optional i32 page_size = 20
}

// Pagination response
struct PageResponse {
    1: required i32 total
    2: required i32 page
    3: required i32 page_size
    4: required bool has_more
}

// Generic response wrapper
struct BaseResponse {
    1: required i32 code
    2: required string message
    3: optional string trace_id
}

// User status enum
enum UserStatus {
    UNKNOWN = 0
    ACTIVE = 1
    INACTIVE = 2
    BANNED = 3
}
