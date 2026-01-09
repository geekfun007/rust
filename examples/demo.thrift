namespace js Demo

typedef i64 UserId

enum Status {
  ACTIVE = 1,
  INACTIVE = 2,
  UNKNOWN // will auto-increment
}

struct User {
  1: required UserId id,
  2: optional string name,
  3: optional i32 age = 18,
  4: optional list<string> tags,
  5: optional map<string, string> meta
}

union LookupKey {
  1: UserId user_id,
  2: string username
}

service UserService {
  User get_user(1: required UserId id),
  void delete_user(1: required UserId id)
}

