//! Common types and AST definitions for Thrift and Proto parsing

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents the file type being parsed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    Thrift,
    Proto,
}

/// Represents a field type in both Thrift and Proto
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldType {
    // Primitive types
    Bool,
    Byte,
    I16,
    I32,
    I64,
    Double,
    Float,
    String,
    Binary,
    // Container types
    List(Box<FieldType>),
    Set(Box<FieldType>),
    Map(Box<FieldType>, Box<FieldType>),
    // User-defined types
    Custom(String),
    // Void (for return types)
    Void,
}

impl FieldType {
    /// Convert to Go type string
    pub fn to_go_type(&self) -> String {
        match self {
            FieldType::Bool => "bool".to_string(),
            FieldType::Byte => "byte".to_string(),
            FieldType::I16 => "int16".to_string(),
            FieldType::I32 => "int32".to_string(),
            FieldType::I64 => "int64".to_string(),
            FieldType::Double => "float64".to_string(),
            FieldType::Float => "float32".to_string(),
            FieldType::String => "string".to_string(),
            FieldType::Binary => "[]byte".to_string(),
            FieldType::List(inner) => format!("[]{}", inner.to_go_type()),
            FieldType::Set(inner) => format!("map[{}]struct{{}}", inner.to_go_type()),
            FieldType::Map(k, v) => format!("map[{}]{}", k.to_go_type(), v.to_go_type()),
            FieldType::Custom(name) => format!("*{}", name),
            FieldType::Void => "".to_string(),
        }
    }
}

/// Represents a field in a struct/message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub id: i32,
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub optional: bool,
    pub default_value: Option<String>,
    pub annotations: HashMap<String, String>,
    pub comments: Vec<String>,
}

/// Represents a struct/message definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<Field>,
    pub annotations: HashMap<String, String>,
    pub comments: Vec<String>,
}

/// Represents an enum value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumValue {
    pub name: String,
    pub value: Option<i32>,
    pub comments: Vec<String>,
}

/// Represents an enum definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDef {
    pub name: String,
    pub values: Vec<EnumValue>,
    pub comments: Vec<String>,
}

/// Represents a method/function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodParam {
    pub id: i32,
    pub name: String,
    pub param_type: FieldType,
}

/// Represents an exception definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExceptionDef {
    pub id: i32,
    pub name: String,
    pub exception_type: FieldType,
}

/// HTTP method annotation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
        }
    }
}

/// HTTP endpoint annotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpEndpoint {
    pub method: HttpMethod,
    pub path: String,
}

/// Represents a service method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub return_type: FieldType,
    pub params: Vec<MethodParam>,
    pub exceptions: Vec<ExceptionDef>,
    pub oneway: bool,
    pub annotations: HashMap<String, String>,
    pub comments: Vec<String>,
    pub http_endpoint: Option<HttpEndpoint>,
}

/// Represents a service definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDef {
    pub name: String,
    pub extends: Option<String>,
    pub methods: Vec<Method>,
    pub annotations: HashMap<String, String>,
    pub comments: Vec<String>,
}

/// Represents a constant definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstDef {
    pub name: String,
    pub const_type: FieldType,
    pub value: String,
    pub comments: Vec<String>,
}

/// Represents a typedef
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedefDef {
    pub name: String,
    pub original_type: FieldType,
    pub comments: Vec<String>,
}

/// Represents an include/import statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncludeDef {
    pub path: String,
    pub alias: Option<String>,
}

/// Represents a parsed IDL file (Thrift or Proto)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedFile {
    pub file_type: FileType,
    pub file_path: PathBuf,
    pub namespace: HashMap<String, String>, // language -> namespace
    pub package: Option<String>,            // For proto
    pub includes: Vec<IncludeDef>,
    pub structs: Vec<StructDef>,
    pub enums: Vec<EnumDef>,
    pub services: Vec<ServiceDef>,
    pub constants: Vec<ConstDef>,
    pub typedefs: Vec<TypedefDef>,
    pub options: HashMap<String, String>, // For proto options
}

impl ParsedFile {
    pub fn new(file_type: FileType, file_path: PathBuf) -> Self {
        Self {
            file_type,
            file_path,
            namespace: HashMap::new(),
            package: None,
            includes: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            services: Vec::new(),
            constants: Vec::new(),
            typedefs: Vec::new(),
            options: HashMap::new(),
        }
    }

    /// Get the Go package name
    pub fn go_package(&self) -> String {
        // First check proto options
        if let Some(pkg) = self.options.get("go_package") {
            return pkg.clone();
        }
        // Then check namespace
        if let Some(ns) = self.namespace.get("go") {
            return ns.clone();
        }
        // Fall back to package name or file name
        self.package.clone().unwrap_or_else(|| {
            self.file_path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "generated".to_string())
        })
    }
}

/// Represents the complete parsed project with all files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedProject {
    pub files: Vec<ParsedFile>,
    pub root_path: PathBuf,
}

impl ParsedProject {
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            files: Vec::new(),
            root_path,
        }
    }

    /// Get all services from all files
    pub fn all_services(&self) -> Vec<&ServiceDef> {
        self.files.iter().flat_map(|f| f.services.iter()).collect()
    }

    /// Get all structs from all files
    pub fn all_structs(&self) -> Vec<&StructDef> {
        self.files.iter().flat_map(|f| f.structs.iter()).collect()
    }

    /// Get all enums from all files
    pub fn all_enums(&self) -> Vec<&EnumDef> {
        self.files.iter().flat_map(|f| f.enums.iter()).collect()
    }
}

/// Token types for lexer
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords - Thrift
    Namespace,
    Include,
    Struct,
    Service,
    Enum,
    Const,
    Typedef,
    Exception,
    Extends,
    Throws,
    Oneway,
    Required,
    Optional,
    
    // Keywords - Proto
    Syntax,
    Package,
    Import,
    Message,
    Rpc,
    Returns,
    Option,
    Repeated,
    Map,
    
    // Primitive types
    Bool,
    Byte,
    I16,
    I32,
    I64,
    Double,
    Float,
    String,
    Binary,
    Void,
    List,
    Set,
    
    // Symbols
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftAngle,
    RightAngle,
    Semicolon,
    Colon,
    Comma,
    Equals,
    Dot,
    
    // Literals
    Identifier(String),
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    
    // Comments
    Comment(String),
    
    // Annotations
    Annotation(String, String),
    
    // End of file
    Eof,
}
