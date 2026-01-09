//! Thrift 语法解析器 (Parser)
//! 
//! 将 Token 流解析为 AST

use crate::ast::*;
use crate::lexer::{Token, TokenKind};
use thiserror::Error;

/// 解析错误
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: expected {expected}, found {found:?} at line {line}")]
    UnexpectedToken {
        expected: String,
        found: TokenKind,
        line: usize,
    },
    
    #[error("Unexpected end of file")]
    UnexpectedEof,
    
    #[error("Invalid field ID at line {0}")]
    InvalidFieldId(usize),
}

/// 解析器
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }
    
    /// 获取当前 Token
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }
    
    /// 获取当前 Token 的类型
    fn current_kind(&self) -> Option<&TokenKind> {
        self.current().map(|t| &t.kind)
    }
    
    /// 前进一个 Token
    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        token
    }
    
    /// 期望特定的 Token
    fn expect(&mut self, expected: TokenKind) -> Result<&Token, ParseError> {
        let token = self.current().ok_or(ParseError::UnexpectedEof)?;
        
        if std::mem::discriminant(&token.kind) == std::mem::discriminant(&expected) {
            Ok(self.advance().unwrap())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: token.kind.clone(),
                line: token.line,
            })
        }
    }
    
    /// 检查当前 Token 是否匹配
    fn check(&self, kind: &TokenKind) -> bool {
        self.current_kind()
            .map(|k| std::mem::discriminant(k) == std::mem::discriminant(kind))
            .unwrap_or(false)
    }
    
    /// 如果匹配则消费 Token
    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    /// 跳过可选的分隔符 (; 或 ,)
    fn skip_separator(&mut self) {
        if self.check(&TokenKind::Semicolon) || self.check(&TokenKind::Comma) {
            self.advance();
        }
    }
    
    /// 解析整个文档
    pub fn parse(&mut self) -> Result<ThriftDocument, ParseError> {
        let mut doc = ThriftDocument::new();
        
        while let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Eof => break,
                
                TokenKind::Namespace => {
                    doc.namespaces.push(self.parse_namespace()?);
                }
                
                TokenKind::Include => {
                    doc.includes.push(self.parse_include()?);
                }
                
                TokenKind::Const => {
                    doc.constants.push(self.parse_const()?);
                }
                
                TokenKind::Typedef => {
                    doc.typedefs.push(self.parse_typedef()?);
                }
                
                TokenKind::Enum => {
                    doc.enums.push(self.parse_enum()?);
                }
                
                TokenKind::Struct => {
                    doc.structs.push(self.parse_struct()?);
                }
                
                TokenKind::Union => {
                    doc.unions.push(self.parse_union()?);
                }
                
                TokenKind::Exception => {
                    doc.exceptions.push(self.parse_exception()?);
                }
                
                TokenKind::Service => {
                    doc.services.push(self.parse_service()?);
                }
                
                _ => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "top-level declaration".to_string(),
                        found: token.kind.clone(),
                        line: token.line,
                    });
                }
            }
        }
        
        Ok(doc)
    }
    
    /// 解析命名空间
    fn parse_namespace(&mut self) -> Result<Namespace, ParseError> {
        self.expect(TokenKind::Namespace)?;
        
        let language = self.parse_identifier()?;
        let name = self.parse_identifier()?;
        
        self.skip_separator();
        
        Ok(Namespace { language, name })
    }
    
    /// 解析 include
    fn parse_include(&mut self) -> Result<Include, ParseError> {
        self.expect(TokenKind::Include)?;
        
        let path = self.parse_string_literal()?;
        
        self.skip_separator();
        
        Ok(Include { path })
    }
    
    /// 解析常量
    fn parse_const(&mut self) -> Result<Constant, ParseError> {
        self.expect(TokenKind::Const)?;
        
        let field_type = self.parse_field_type()?;
        let name = self.parse_identifier()?;
        
        self.expect(TokenKind::Equals)?;
        
        let value = self.parse_const_value()?;
        
        self.skip_separator();
        
        Ok(Constant {
            field_type,
            name,
            value,
        })
    }
    
    /// 解析 typedef
    fn parse_typedef(&mut self) -> Result<Typedef, ParseError> {
        self.expect(TokenKind::Typedef)?;
        
        let original_type = self.parse_field_type()?;
        let alias = self.parse_identifier()?;
        
        self.skip_separator();
        
        Ok(Typedef {
            original_type,
            alias,
        })
    }
    
    /// 解析枚举
    fn parse_enum(&mut self) -> Result<Enum, ParseError> {
        self.expect(TokenKind::Enum)?;
        
        let name = self.parse_identifier()?;
        
        self.expect(TokenKind::LeftBrace)?;
        
        let mut values = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) {
            let value_name = self.parse_identifier()?;
            
            let value = if self.match_token(&TokenKind::Equals) {
                Some(self.parse_int_literal()?)
            } else {
                None
            };
            
            values.push(EnumValue {
                name: value_name,
                value,
            });
            
            self.skip_separator();
        }
        
        self.expect(TokenKind::RightBrace)?;
        
        Ok(Enum { name, values })
    }
    
    /// 解析结构体
    fn parse_struct(&mut self) -> Result<Struct, ParseError> {
        self.expect(TokenKind::Struct)?;
        
        let name = self.parse_identifier()?;
        
        self.expect(TokenKind::LeftBrace)?;
        
        let fields = self.parse_fields()?;
        
        self.expect(TokenKind::RightBrace)?;
        
        Ok(Struct { name, fields })
    }
    
    /// 解析 union
    fn parse_union(&mut self) -> Result<Union, ParseError> {
        self.expect(TokenKind::Union)?;
        
        let name = self.parse_identifier()?;
        
        self.expect(TokenKind::LeftBrace)?;
        
        let fields = self.parse_fields()?;
        
        self.expect(TokenKind::RightBrace)?;
        
        Ok(Union { name, fields })
    }
    
    /// 解析异常
    fn parse_exception(&mut self) -> Result<Exception, ParseError> {
        self.expect(TokenKind::Exception)?;
        
        let name = self.parse_identifier()?;
        
        self.expect(TokenKind::LeftBrace)?;
        
        let fields = self.parse_fields()?;
        
        self.expect(TokenKind::RightBrace)?;
        
        Ok(Exception { name, fields })
    }
    
    /// 解析服务
    fn parse_service(&mut self) -> Result<Service, ParseError> {
        self.expect(TokenKind::Service)?;
        
        let name = self.parse_identifier()?;
        
        let extends = if self.match_token(&TokenKind::Extends) {
            Some(self.parse_identifier()?)
        } else {
            None
        };
        
        self.expect(TokenKind::LeftBrace)?;
        
        let mut methods = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) {
            methods.push(self.parse_method()?);
        }
        
        self.expect(TokenKind::RightBrace)?;
        
        Ok(Service {
            name,
            extends,
            methods,
        })
    }
    
    /// 解析服务方法
    fn parse_method(&mut self) -> Result<Method, ParseError> {
        let oneway = self.match_token(&TokenKind::Oneway);
        
        let return_type = self.parse_field_type()?;
        let name = self.parse_identifier()?;
        
        self.expect(TokenKind::LeftParen)?;
        
        let arguments = self.parse_fields()?;
        
        self.expect(TokenKind::RightParen)?;
        
        let throws = if self.match_token(&TokenKind::Throws) {
            self.expect(TokenKind::LeftParen)?;
            let throws_fields = self.parse_fields()?;
            self.expect(TokenKind::RightParen)?;
            throws_fields
        } else {
            Vec::new()
        };
        
        self.skip_separator();
        
        Ok(Method {
            oneway,
            return_type,
            name,
            arguments,
            throws,
        })
    }
    
    /// 解析字段列表
    fn parse_fields(&mut self) -> Result<Vec<Field>, ParseError> {
        let mut fields = Vec::new();
        
        while !self.check(&TokenKind::RightBrace) && !self.check(&TokenKind::RightParen) {
            fields.push(self.parse_field()?);
        }
        
        Ok(fields)
    }
    
    /// 解析单个字段
    fn parse_field(&mut self) -> Result<Field, ParseError> {
        // 可选的字段 ID
        let id = if let Some(TokenKind::IntLiteral(n)) = self.current_kind().cloned() {
            self.advance();
            self.expect(TokenKind::Colon)?;
            Some(n as i32)
        } else {
            None
        };
        
        // 可选的 required/optional
        let requiredness = if self.match_token(&TokenKind::Required) {
            Requiredness::Required
        } else if self.match_token(&TokenKind::Optional) {
            Requiredness::Optional
        } else {
            Requiredness::Default
        };
        
        let field_type = self.parse_field_type()?;
        let name = self.parse_identifier()?;
        
        // 可选的默认值
        let default_value = if self.match_token(&TokenKind::Equals) {
            Some(self.parse_const_value()?)
        } else {
            None
        };
        
        self.skip_separator();
        
        Ok(Field {
            id,
            requiredness,
            field_type,
            name,
            default_value,
        })
    }
    
    /// 解析字段类型
    fn parse_field_type(&mut self) -> Result<FieldType, ParseError> {
        let token = self.current().ok_or(ParseError::UnexpectedEof)?;
        
        let field_type = match &token.kind {
            TokenKind::Bool => {
                self.advance();
                FieldType::Bool
            }
            TokenKind::Byte => {
                self.advance();
                FieldType::Byte
            }
            TokenKind::I16 => {
                self.advance();
                FieldType::I16
            }
            TokenKind::I32 => {
                self.advance();
                FieldType::I32
            }
            TokenKind::I64 => {
                self.advance();
                FieldType::I64
            }
            TokenKind::Double => {
                self.advance();
                FieldType::Double
            }
            TokenKind::String => {
                self.advance();
                FieldType::String
            }
            TokenKind::Binary => {
                self.advance();
                FieldType::Binary
            }
            TokenKind::Void => {
                self.advance();
                FieldType::Void
            }
            TokenKind::List => {
                self.advance();
                self.expect(TokenKind::LeftAngle)?;
                let inner = self.parse_field_type()?;
                self.expect(TokenKind::RightAngle)?;
                FieldType::List(Box::new(inner))
            }
            TokenKind::Set => {
                self.advance();
                self.expect(TokenKind::LeftAngle)?;
                let inner = self.parse_field_type()?;
                self.expect(TokenKind::RightAngle)?;
                FieldType::Set(Box::new(inner))
            }
            TokenKind::Map => {
                self.advance();
                self.expect(TokenKind::LeftAngle)?;
                let key = self.parse_field_type()?;
                self.expect(TokenKind::Comma)?;
                let value = self.parse_field_type()?;
                self.expect(TokenKind::RightAngle)?;
                FieldType::Map(Box::new(key), Box::new(value))
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                FieldType::Custom(name)
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "type".to_string(),
                    found: token.kind.clone(),
                    line: token.line,
                });
            }
        };
        
        Ok(field_type)
    }
    
    /// 解析常量值
    fn parse_const_value(&mut self) -> Result<ConstValue, ParseError> {
        let token = self.current().ok_or(ParseError::UnexpectedEof)?;
        
        let value = match &token.kind {
            TokenKind::BoolLiteral(b) => {
                let b = *b;
                self.advance();
                ConstValue::Bool(b)
            }
            TokenKind::IntLiteral(n) => {
                let n = *n;
                self.advance();
                ConstValue::Int(n)
            }
            TokenKind::DoubleLiteral(n) => {
                let n = *n;
                self.advance();
                ConstValue::Double(n)
            }
            TokenKind::StringLiteral(s) => {
                let s = s.clone();
                self.advance();
                ConstValue::String(s)
            }
            TokenKind::Identifier(s) => {
                let s = s.clone();
                self.advance();
                ConstValue::Identifier(s)
            }
            TokenKind::LeftBracket => {
                self.advance();
                let mut items = Vec::new();
                
                while !self.check(&TokenKind::RightBracket) {
                    items.push(self.parse_const_value()?);
                    self.skip_separator();
                }
                
                self.expect(TokenKind::RightBracket)?;
                ConstValue::List(items)
            }
            TokenKind::LeftBrace => {
                self.advance();
                let mut items = Vec::new();
                
                while !self.check(&TokenKind::RightBrace) {
                    let key = self.parse_const_value()?;
                    self.expect(TokenKind::Colon)?;
                    let value = self.parse_const_value()?;
                    items.push((key, value));
                    self.skip_separator();
                }
                
                self.expect(TokenKind::RightBrace)?;
                ConstValue::Map(items)
            }
            _ => {
                return Err(ParseError::UnexpectedToken {
                    expected: "constant value".to_string(),
                    found: token.kind.clone(),
                    line: token.line,
                });
            }
        };
        
        Ok(value)
    }
    
    /// 解析标识符
    fn parse_identifier(&mut self) -> Result<String, ParseError> {
        let token = self.current().ok_or(ParseError::UnexpectedEof)?;
        
        if let TokenKind::Identifier(name) = &token.kind {
            let name = name.clone();
            self.advance();
            Ok(name)
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: token.kind.clone(),
                line: token.line,
            })
        }
    }
    
    /// 解析字符串字面量
    fn parse_string_literal(&mut self) -> Result<String, ParseError> {
        let token = self.current().ok_or(ParseError::UnexpectedEof)?;
        
        if let TokenKind::StringLiteral(s) = &token.kind {
            let s = s.clone();
            self.advance();
            Ok(s)
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "string literal".to_string(),
                found: token.kind.clone(),
                line: token.line,
            })
        }
    }
    
    /// 解析整数字面量
    fn parse_int_literal(&mut self) -> Result<i64, ParseError> {
        let token = self.current().ok_or(ParseError::UnexpectedEof)?;
        
        if let TokenKind::IntLiteral(n) = &token.kind {
            let n = *n;
            self.advance();
            Ok(n)
        } else {
            Err(ParseError::UnexpectedToken {
                expected: "integer literal".to_string(),
                found: token.kind.clone(),
                line: token.line,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    
    #[test]
    fn test_parse_struct() {
        let input = r#"
            struct User {
                1: required i64 id
                2: required string name
                3: optional string email
            }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();
        
        assert_eq!(doc.structs.len(), 1);
        assert_eq!(doc.structs[0].name, "User");
        assert_eq!(doc.structs[0].fields.len(), 3);
    }
    
    #[test]
    fn test_parse_enum() {
        let input = r#"
            enum Status {
                ACTIVE = 1,
                INACTIVE = 2,
                DELETED = 3
            }
        "#;
        
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let doc = parser.parse().unwrap();
        
        assert_eq!(doc.enums.len(), 1);
        assert_eq!(doc.enums[0].name, "Status");
        assert_eq!(doc.enums[0].values.len(), 3);
    }
}
