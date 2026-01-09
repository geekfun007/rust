//! Thrift 词法分析器 (Lexer)
//! 
//! 将 Thrift IDL 源代码转换为 Token 流

use std::iter::Peekable;
use std::str::Chars;
use thiserror::Error;

/// 词法分析错误
#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Unexpected character '{0}' at line {1}, column {2}")]
    UnexpectedChar(char, usize, usize),
    
    #[error("Unterminated string at line {0}")]
    UnterminatedString(usize),
    
    #[error("Invalid number format at line {0}")]
    InvalidNumber(usize),
}

/// Token 类型
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // 关键字
    Namespace,
    Include,
    Struct,
    Service,
    Enum,
    Const,
    Typedef,
    Exception,
    Union,
    Required,
    Optional,
    Oneway,
    Throws,
    Extends,
    
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
    List,
    Set,
    Map,
    
    // 标点符号
    LeftBrace,    // {
    RightBrace,   // }
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftAngle,    // <
    RightAngle,   // >
    Colon,        // :
    Semicolon,    // ;
    Comma,        // ,
    Equals,       // =
    Dot,          // .
    
    // 字面量
    Identifier(String),
    IntLiteral(i64),
    DoubleLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    
    // 特殊
    Comment(String),
    Eof,
}

/// Token 结构
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}

/// 词法分析器
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input: input.chars().peekable(),
            line: 1,
            column: 0,
            current_char: None,
        };
        lexer.advance();
        lexer
    }
    
    /// 前进一个字符
    fn advance(&mut self) -> Option<char> {
        let prev = self.current_char;
        self.current_char = self.input.next();
        
        if let Some(c) = self.current_char {
            if c == '\n' {
                self.line += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
        
        prev
    }
    
    /// 查看当前字符
    fn current(&self) -> Option<char> {
        self.current_char
    }
    
    /// 查看下一个字符
    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }
    
    /// 跳过空白字符
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }
    
    /// 跳过单行注释
    fn skip_line_comment(&mut self) -> String {
        let mut comment = String::new();
        // 跳过 // 或 #
        self.advance();
        if self.current() == Some('/') || self.current() == Some('#') {
            self.advance();
        }
        
        while let Some(c) = self.current() {
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.advance();
        }
        comment.trim().to_string()
    }
    
    /// 跳过多行注释
    fn skip_block_comment(&mut self) -> String {
        let mut comment = String::new();
        // 跳过 /*
        self.advance();
        self.advance();
        
        while let Some(c) = self.current() {
            if c == '*' {
                if self.peek() == Some(&'/') {
                    self.advance();
                    self.advance();
                    break;
                }
            }
            comment.push(c);
            self.advance();
        }
        comment.trim().to_string()
    }
    
    /// 读取标识符或关键字
    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        
        while let Some(c) = self.current() {
            if c.is_alphanumeric() || c == '_' || c == '.' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        
        ident
    }
    
    /// 读取数字
    fn read_number(&mut self) -> Result<TokenKind, LexerError> {
        let mut num_str = String::new();
        let mut is_float = false;
        let is_negative = self.current() == Some('-');
        
        if is_negative {
            num_str.push('-');
            self.advance();
        }
        
        while let Some(c) = self.current() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.advance();
            } else if c == '.' && !is_float {
                is_float = true;
                num_str.push(c);
                self.advance();
            } else if c == 'e' || c == 'E' {
                is_float = true;
                num_str.push(c);
                self.advance();
                if self.current() == Some('+') || self.current() == Some('-') {
                    num_str.push(self.current().unwrap());
                    self.advance();
                }
            } else {
                break;
            }
        }
        
        if is_float {
            num_str.parse::<f64>()
                .map(TokenKind::DoubleLiteral)
                .map_err(|_| LexerError::InvalidNumber(self.line))
        } else {
            num_str.parse::<i64>()
                .map(TokenKind::IntLiteral)
                .map_err(|_| LexerError::InvalidNumber(self.line))
        }
    }
    
    /// 读取字符串字面量
    fn read_string(&mut self) -> Result<String, LexerError> {
        let quote = self.current().unwrap();
        let start_line = self.line;
        self.advance(); // 跳过开始引号
        
        let mut string = String::new();
        
        while let Some(c) = self.current() {
            if c == quote {
                self.advance(); // 跳过结束引号
                return Ok(string);
            } else if c == '\\' {
                self.advance();
                if let Some(escaped) = self.current() {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        '\'' => string.push('\''),
                        _ => {
                            string.push('\\');
                            string.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                string.push(c);
                self.advance();
            }
        }
        
        Err(LexerError::UnterminatedString(start_line))
    }
    
    /// 将标识符转换为关键字或标识符 Token
    fn identifier_to_token(&self, ident: &str) -> TokenKind {
        match ident {
            // 关键字
            "namespace" => TokenKind::Namespace,
            "include" => TokenKind::Include,
            "struct" => TokenKind::Struct,
            "service" => TokenKind::Service,
            "enum" => TokenKind::Enum,
            "const" => TokenKind::Const,
            "typedef" => TokenKind::Typedef,
            "exception" => TokenKind::Exception,
            "union" => TokenKind::Union,
            "required" => TokenKind::Required,
            "optional" => TokenKind::Optional,
            "oneway" => TokenKind::Oneway,
            "throws" => TokenKind::Throws,
            "extends" => TokenKind::Extends,
            
            // 基本类型
            "bool" => TokenKind::Bool,
            "byte" => TokenKind::Byte,
            "i16" => TokenKind::I16,
            "i32" => TokenKind::I32,
            "i64" => TokenKind::I64,
            "double" => TokenKind::Double,
            "string" => TokenKind::String,
            "binary" => TokenKind::Binary,
            "void" => TokenKind::Void,
            
            // 容器类型
            "list" => TokenKind::List,
            "set" => TokenKind::Set,
            "map" => TokenKind::Map,
            
            // 布尔字面量
            "true" => TokenKind::BoolLiteral(true),
            "false" => TokenKind::BoolLiteral(false),
            
            // 标识符
            _ => TokenKind::Identifier(ident.to_string()),
        }
    }
    
    /// 获取下一个 Token
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        
        let line = self.line;
        let column = self.column;
        
        let kind = match self.current() {
            None => TokenKind::Eof,
            
            Some(c) => match c {
                // 注释
                '/' => {
                    if self.peek() == Some(&'/') {
                        let comment = self.skip_line_comment();
                        TokenKind::Comment(comment)
                    } else if self.peek() == Some(&'*') {
                        let comment = self.skip_block_comment();
                        TokenKind::Comment(comment)
                    } else {
                        return Err(LexerError::UnexpectedChar(c, line, column));
                    }
                }
                
                '#' => {
                    let comment = self.skip_line_comment();
                    TokenKind::Comment(comment)
                }
                
                // 标点符号
                '{' => { self.advance(); TokenKind::LeftBrace }
                '}' => { self.advance(); TokenKind::RightBrace }
                '(' => { self.advance(); TokenKind::LeftParen }
                ')' => { self.advance(); TokenKind::RightParen }
                '[' => { self.advance(); TokenKind::LeftBracket }
                ']' => { self.advance(); TokenKind::RightBracket }
                '<' => { self.advance(); TokenKind::LeftAngle }
                '>' => { self.advance(); TokenKind::RightAngle }
                ':' => { self.advance(); TokenKind::Colon }
                ';' => { self.advance(); TokenKind::Semicolon }
                ',' => { self.advance(); TokenKind::Comma }
                '=' => { self.advance(); TokenKind::Equals }
                '.' => { self.advance(); TokenKind::Dot }
                
                // 字符串
                '"' | '\'' => {
                    let s = self.read_string()?;
                    TokenKind::StringLiteral(s)
                }
                
                // 数字
                '0'..='9' => self.read_number()?,
                
                '-' if self.peek().map_or(false, |c| c.is_ascii_digit()) => {
                    self.read_number()?
                }
                
                // 标识符或关键字
                c if c.is_alphabetic() || c == '_' => {
                    let ident = self.read_identifier();
                    self.identifier_to_token(&ident)
                }
                
                _ => return Err(LexerError::UnexpectedChar(c, line, column)),
            }
        };
        
        Ok(Token::new(kind, line, column))
    }
    
    /// 将所有 Token 收集到 Vec 中
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.kind, TokenKind::Eof);
            
            // 跳过注释
            if !matches!(token.kind, TokenKind::Comment(_)) {
                tokens.push(token);
            }
            
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_tokens() {
        let input = "struct User { }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].kind, TokenKind::Struct));
        assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "User"));
        assert!(matches!(tokens[2].kind, TokenKind::LeftBrace));
        assert!(matches!(tokens[3].kind, TokenKind::RightBrace));
    }
    
    #[test]
    fn test_string_literal() {
        let input = r#""hello world""#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].kind, TokenKind::StringLiteral(ref s) if s == "hello world"));
    }
    
    #[test]
    fn test_numbers() {
        let input = "123 -456 3.14";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].kind, TokenKind::IntLiteral(123)));
        assert!(matches!(tokens[1].kind, TokenKind::IntLiteral(-456)));
        assert!(matches!(tokens[2].kind, TokenKind::DoubleLiteral(f) if (f - 3.14).abs() < 0.001));
    }
}
