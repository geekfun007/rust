// 用户服务 Thrift 定义示例

namespace js user
namespace py user

// 用户状态枚举
enum UserStatus {
    ACTIVE = 1,
    INACTIVE = 2,
    SUSPENDED = 3,
    DELETED = 4
}

// 性别枚举
enum Gender {
    UNKNOWN = 0,
    MALE = 1,
    FEMALE = 2
}

// 用户地址结构
struct Address {
    1: required string country
    2: required string city
    3: optional string street
    4: optional string zipCode
}

// 用户配置
struct UserProfile {
    1: optional string avatar
    2: optional string bio
    3: optional list<string> interests
    4: optional map<string, string> socialLinks
}

// 用户结构
struct User {
    1: required i64 id
    2: required string username
    3: required string email
    4: optional string phone
    5: required UserStatus status
    6: optional Gender gender
    7: optional Address address
    8: optional UserProfile profile
    9: required i64 createdAt
    10: optional i64 updatedAt
}

// 创建用户请求
struct CreateUserRequest {
    1: required string username
    2: required string email
    3: optional string phone
    4: optional Gender gender
    5: optional Address address
}

// 创建用户响应
struct CreateUserResponse {
    1: required User user
    2: required string token
}

// 更新用户请求
struct UpdateUserRequest {
    1: required i64 id
    2: optional string email
    3: optional string phone
    4: optional Gender gender
    5: optional Address address
    6: optional UserProfile profile
}

// 分页参数
struct Pagination {
    1: required i32 page
    2: required i32 pageSize
}

// 用户列表响应
struct UserListResponse {
    1: required list<User> users
    2: required i32 total
    3: required i32 page
    4: required i32 pageSize
}

// 用户未找到异常
exception UserNotFoundException {
    1: required i64 userId
    2: required string message
}

// 用户已存在异常
exception UserAlreadyExistsException {
    1: required string email
    2: required string message
}

// 用户服务接口
service UserService {
    // 获取单个用户
    User getUser(1: i64 id) throws (1: UserNotFoundException e)
    
    // 获取用户列表
    UserListResponse listUsers(1: Pagination pagination)
    
    // 创建用户
    CreateUserResponse createUser(1: CreateUserRequest request) 
        throws (1: UserAlreadyExistsException e)
    
    // 更新用户
    User updateUser(1: UpdateUserRequest request) 
        throws (1: UserNotFoundException e)
    
    // 删除用户
    void deleteUser(1: i64 id) throws (1: UserNotFoundException e)
    
    // 搜索用户
    UserListResponse searchUsers(1: string keyword, 2: Pagination pagination)
}
