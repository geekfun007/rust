// User service definition
namespace go example.user

include "common.thrift"

// User entity
struct User {
    1: required i64 id
    2: required string username
    3: optional string email
    4: optional string avatar
    5: required common.UserStatus status
    6: required i64 created_at
    7: optional i64 updated_at
}

// Request/Response messages
struct GetUserRequest {
    1: required i64 id (api.path = "id")
}

struct GetUserResponse {
    1: required common.BaseResponse base
    2: optional User user
}

struct CreateUserRequest {
    1: required string username (api.body = "username")
    2: required string email (api.body = "email")
    3: optional string password (api.body = "password")
}

struct CreateUserResponse {
    1: required common.BaseResponse base
    2: optional User user
}

struct UpdateUserRequest {
    1: required i64 id (api.path = "id")
    2: optional string username (api.body = "username")
    3: optional string email (api.body = "email")
    4: optional string avatar (api.body = "avatar")
}

struct UpdateUserResponse {
    1: required common.BaseResponse base
    2: optional User user
}

struct DeleteUserRequest {
    1: required i64 id (api.path = "id")
}

struct DeleteUserResponse {
    1: required common.BaseResponse base
}

struct ListUsersRequest {
    1: required common.PageRequest page
    2: optional common.UserStatus status (api.query = "status")
}

struct ListUsersResponse {
    1: required common.BaseResponse base
    2: required list<User> users
    3: required common.PageResponse page
}

// User service
service UserService {
    GetUserResponse GetUser(1: GetUserRequest req) (api.get = "/api/v1/users/:id")
    CreateUserResponse CreateUser(1: CreateUserRequest req) (api.post = "/api/v1/users")
    UpdateUserResponse UpdateUser(1: UpdateUserRequest req) (api.put = "/api/v1/users/:id")
    DeleteUserResponse DeleteUser(1: DeleteUserRequest req) (api.delete = "/api/v1/users/:id")
    ListUsersResponse ListUsers(1: ListUsersRequest req) (api.get = "/api/v1/users")
}
