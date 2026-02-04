// Order service definition
namespace go example.order

include "common.thrift"

// Order status
enum OrderStatus {
    PENDING = 0
    PAID = 1
    SHIPPED = 2
    DELIVERED = 3
    CANCELLED = 4
    REFUNDED = 5
}

// Order item
struct OrderItem {
    1: required i64 product_id
    2: required string product_name
    3: required i32 quantity
    4: required double price
    5: optional double discount
}

// Order entity
struct Order {
    1: required i64 id
    2: required i64 user_id
    3: required list<OrderItem> items
    4: required double total_amount
    5: required OrderStatus status
    6: optional string shipping_address
    7: optional string note
    8: required i64 created_at
    9: optional i64 updated_at
}

// Request/Response messages
struct CreateOrderRequest {
    1: required i64 user_id (api.body = "user_id")
    2: required list<OrderItem> items (api.body = "items")
    3: optional string shipping_address (api.body = "shipping_address")
    4: optional string note (api.body = "note")
}

struct CreateOrderResponse {
    1: required common.BaseResponse base
    2: optional Order order
}

struct GetOrderRequest {
    1: required i64 id (api.path = "id")
}

struct GetOrderResponse {
    1: required common.BaseResponse base
    2: optional Order order
}

struct UpdateOrderStatusRequest {
    1: required i64 id (api.path = "id")
    2: required OrderStatus status (api.body = "status")
}

struct UpdateOrderStatusResponse {
    1: required common.BaseResponse base
    2: optional Order order
}

struct ListOrdersRequest {
    1: required i64 user_id (api.query = "user_id")
    2: optional OrderStatus status (api.query = "status")
    3: required common.PageRequest page
}

struct ListOrdersResponse {
    1: required common.BaseResponse base
    2: required list<Order> orders
    3: required common.PageResponse page
}

struct CancelOrderRequest {
    1: required i64 id (api.path = "id")
    2: optional string reason (api.body = "reason")
}

struct CancelOrderResponse {
    1: required common.BaseResponse base
    2: optional Order order
}

// Order service
service OrderService {
    CreateOrderResponse CreateOrder(1: CreateOrderRequest req) (api.post = "/api/v1/orders")
    GetOrderResponse GetOrder(1: GetOrderRequest req) (api.get = "/api/v1/orders/:id")
    UpdateOrderStatusResponse UpdateOrderStatus(1: UpdateOrderStatusRequest req) (api.patch = "/api/v1/orders/:id/status")
    ListOrdersResponse ListOrders(1: ListOrdersRequest req) (api.get = "/api/v1/orders")
    CancelOrderResponse CancelOrder(1: CancelOrderRequest req) (api.post = "/api/v1/orders/:id/cancel")
}
