//! Thrift AST (抽象语法树) 定义
//! 
//! 定义 Thrift IDL 的所有语法结构

/// Thrift 文档 (顶层结构)
#[derive(Debug, Clone)]
pub struct ThriftDocument {
    /// 命名空间声明
    pub namespaces: Vec<Namespace>,
    /// Include 声明
    pub includes: Vec<Include>,
    /// 常量定义
    pub constants: Vec<Constant>,
    /// 类型定义 (typedef)
    pub typedefs: Vec<Typedef>,
    /// 枚举定义
    pub enums: Vec<Enum>,
    /// 结构体定义
    pub structs: Vec<Struct>,
    /// Union 定义
    pub unions: Vec<Union>,
    /// 异常定义
    pub exceptions: Vec<Exception>,
    /// 服务定义
    pub services: Vec<Service>,
}

impl ThriftDocument {
    pub fn new() -> Self {
        Self {
            namespaces: Vec::new(),
            includes: Vec::new(),
            constants: Vec::new(),
            typedefs: Vec::new(),
            enums: Vec::new(),
            structs: Vec::new(),
            unions: Vec::new(),
            exceptions: Vec::new(),
            services: Vec::new(),
        }
    }
}

impl Default for ThriftDocument {
    fn default() -> Self {
        Self::new()
    }
}

/// 命名空间声明
#[derive(Debug, Clone)]
pub struct Namespace {
    /// 语言 (如 "js", "cpp", "java")
    pub language: String,
    /// 命名空间名称
    pub name: String,
}

/// Include 声明
#[derive(Debug, Clone)]
pub struct Include {
    /// 包含的文件路径
    pub path: String,
}

/// 常量定义
#[derive(Debug, Clone)]
pub struct Constant {
    /// 类型
    pub field_type: FieldType,
    /// 名称
    pub name: String,
    /// 值
    pub value: ConstValue,
}

/// 类型定义 (typedef)
#[derive(Debug, Clone)]
pub struct Typedef {
    /// 原始类型
    pub original_type: FieldType,
    /// 新类型名称
    pub alias: String,
}

/// 枚举定义
#[derive(Debug, Clone)]
pub struct Enum {
    /// 枚举名称
    pub name: String,
    /// 枚举值列表
    pub values: Vec<EnumValue>,
}

/// 枚举值
#[derive(Debug, Clone)]
pub struct EnumValue {
    /// 值名称
    pub name: String,
    /// 可选的数值
    pub value: Option<i64>,
}

/// 结构体定义
#[derive(Debug, Clone)]
pub struct Struct {
    /// 结构体名称
    pub name: String,
    /// 字段列表
    pub fields: Vec<Field>,
}

/// Union 定义
#[derive(Debug, Clone)]
pub struct Union {
    /// Union 名称
    pub name: String,
    /// 字段列表
    pub fields: Vec<Field>,
}

/// 异常定义
#[derive(Debug, Clone)]
pub struct Exception {
    /// 异常名称
    pub name: String,
    /// 字段列表
    pub fields: Vec<Field>,
}

/// 服务定义
#[derive(Debug, Clone)]
pub struct Service {
    /// 服务名称
    pub name: String,
    /// 继承的服务
    pub extends: Option<String>,
    /// 方法列表
    pub methods: Vec<Method>,
}

/// 服务方法
#[derive(Debug, Clone)]
pub struct Method {
    /// 是否为 oneway
    pub oneway: bool,
    /// 返回类型
    pub return_type: FieldType,
    /// 方法名
    pub name: String,
    /// 参数列表
    pub arguments: Vec<Field>,
    /// 异常列表
    pub throws: Vec<Field>,
}

/// 字段定义
#[derive(Debug, Clone)]
pub struct Field {
    /// 字段 ID
    pub id: Option<i32>,
    /// 字段修饰符 (required/optional)
    pub requiredness: Requiredness,
    /// 字段类型
    pub field_type: FieldType,
    /// 字段名
    pub name: String,
    /// 默认值
    pub default_value: Option<ConstValue>,
}

/// 字段必要性修饰符
#[derive(Debug, Clone, PartialEq)]
pub enum Requiredness {
    Required,
    Optional,
    Default,
}

/// 字段类型
#[derive(Debug, Clone)]
pub enum FieldType {
    // 基本类型
    Bool,
    Byte,
    I16,
    I32,
    I64,
    Double,
    String,
    Binary,
    Void,
    
    // 容器类型
    List(Box<FieldType>),
    Set(Box<FieldType>),
    Map(Box<FieldType>, Box<FieldType>),
    
    // 自定义类型 (引用其他 struct/enum/typedef)
    Custom(String),
}

/// 常量值
#[derive(Debug, Clone)]
pub enum ConstValue {
    Bool(bool),
    Int(i64),
    Double(f64),
    String(String),
    List(Vec<ConstValue>),
    Map(Vec<(ConstValue, ConstValue)>),
    Identifier(String),
}
