//! Protocol Buffers parser implementation

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_until, take_while1},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1, one_of},
    combinator::{map, opt, recognize, value},
    error::VerboseError,
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use std::collections::HashMap;
use std::path::PathBuf;

use super::types::*;

type ParseResult<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

/// Parse whitespace and comments
fn ws(input: &str) -> ParseResult<&str> {
    let (input, _) = multispace0(input)?;
    let (input, _) = many0(alt((
        preceded(tag("//"), take_until("\n")),
        delimited(tag("/*"), take_until("*/"), tag("*/")),
        multispace1,
    )))(input)?;
    Ok((input, ""))
}

/// Parse an identifier
fn identifier(input: &str) -> ParseResult<String> {
    let (input, _) = ws(input)?;
    let (input, id) = recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)?;
    Ok((input, id.to_string()))
}

/// Parse a fully qualified identifier (with dots)
fn fq_identifier(input: &str) -> ParseResult<String> {
    let (input, _) = ws(input)?;
    let (input, id) = recognize(pair(
        opt(char('.')),
        pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_"), tag(".")))),
        ),
    ))(input)?;
    Ok((input, id.to_string()))
}

/// Parse a string literal
fn string_literal(input: &str) -> ParseResult<String> {
    let (input, _) = ws(input)?;
    let (input, s) = delimited(
        char('"'),
        map(
            opt(escaped(
                take_while1(|c| c != '"' && c != '\\'),
                '\\',
                one_of("\"\\nrt"),
            )),
            |o| o.unwrap_or(""),
        ),
        char('"'),
    )(input)?;
    Ok((input, s.to_string()))
}

/// Parse an integer literal
fn int_literal(input: &str) -> ParseResult<i64> {
    let (input, _) = ws(input)?;
    let (input, sign) = opt(alt((char('-'), char('+'))))(input)?;
    let (input, digits) = digit1(input)?;
    let value: i64 = digits.parse().unwrap_or(0);
    let value = if sign == Some('-') { -value } else { value };
    Ok((input, value))
}

/// Parse syntax declaration
fn parse_syntax(input: &str) -> ParseResult<String> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("syntax")(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, version) = string_literal(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, version))
}

/// Parse package declaration
fn parse_package(input: &str) -> ParseResult<String> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("package")(input)?;
    let (input, pkg) = fq_identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, pkg))
}

/// Parse import statement
fn parse_import(input: &str) -> ParseResult<IncludeDef> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("import")(input)?;
    let (input, _) = ws(input)?;
    let (input, _public) = opt(alt((tag("public"), tag("weak"))))(input)?;
    let (input, path) = string_literal(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, IncludeDef { path, alias: None }))
}

/// Parse option statement
fn parse_option(input: &str) -> ParseResult<(String, String)> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("option")(input)?;
    let (input, name) = fq_identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = alt((
        string_literal,
        map(fq_identifier, |s| s),
        map(int_literal, |i| i.to_string()),
    ))(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, (name, value)))
}

/// Parse a field type
fn parse_field_type(input: &str) -> ParseResult<FieldType> {
    let (input, _) = ws(input)?;
    alt((
        // Map type
        map(
            preceded(
                tag("map"),
                delimited(
                    preceded(ws, char('<')),
                    tuple((
                        parse_field_type,
                        preceded(preceded(ws, char(',')), parse_field_type),
                    )),
                    preceded(ws, char('>')),
                ),
            ),
            |(k, v)| FieldType::Map(Box::new(k), Box::new(v)),
        ),
        // Primitive types
        value(FieldType::Bool, tag("bool")),
        value(FieldType::I32, tag("int32")),
        value(FieldType::I64, tag("int64")),
        value(FieldType::I32, tag("uint32")),
        value(FieldType::I64, tag("uint64")),
        value(FieldType::I32, tag("sint32")),
        value(FieldType::I64, tag("sint64")),
        value(FieldType::I32, tag("fixed32")),
        value(FieldType::I64, tag("fixed64")),
        value(FieldType::I32, tag("sfixed32")),
        value(FieldType::I64, tag("sfixed64")),
        value(FieldType::Double, tag("double")),
        value(FieldType::Float, tag("float")),
        value(FieldType::String, tag("string")),
        value(FieldType::Binary, tag("bytes")),
        // Custom types (must be last)
        map(fq_identifier, |s| FieldType::Custom(s)),
    ))(input)
}

/// Parse field options
fn parse_field_options(input: &str) -> ParseResult<HashMap<String, String>> {
    let (input, _) = ws(input)?;
    let (input, options) = opt(delimited(
        char('['),
        separated_list0(
            preceded(ws, char(',')),
            tuple((
                preceded(ws, fq_identifier),
                preceded(preceded(ws, char('=')), alt((
                    string_literal,
                    map(fq_identifier, |s| s),
                    map(int_literal, |i| i.to_string()),
                ))),
            )),
        ),
        preceded(ws, char(']')),
    ))(input)?;

    let mut map = HashMap::new();
    if let Some(opts) = options {
        for (key, value) in opts {
            map.insert(key, value);
        }
    }
    Ok((input, map))
}

/// Parse a message field
fn parse_message_field(input: &str) -> ParseResult<Field> {
    let (input, _) = ws(input)?;
    let (input, repeated) = opt(terminated(tag("repeated"), multispace1))(input)?;
    let (input, optional) = opt(terminated(tag("optional"), multispace1))(input)?;
    let (input, field_type) = parse_field_type(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, id) = int_literal(input)?;
    let (input, annotations) = parse_field_options(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';')(input)?;

    let final_type = if repeated.is_some() {
        FieldType::List(Box::new(field_type))
    } else {
        field_type
    };

    Ok((
        input,
        Field {
            id: id as i32,
            name,
            field_type: final_type,
            required: false,
            optional: optional.is_some(),
            default_value: None,
            annotations,
            comments: Vec::new(),
        },
    ))
}

/// Parse reserved statement
fn parse_reserved(input: &str) -> ParseResult<()> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("reserved")(input)?;
    let (input, _) = take_until(";")(input)?;
    let (input, _) = char(';')(input)?;
    Ok((input, ()))
}

/// Parse a nested enum inside message
fn parse_nested_enum(input: &str) -> ParseResult<EnumDef> {
    parse_enum(input)
}

/// Parse a message definition
fn parse_message(input: &str) -> ParseResult<StructDef> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("message")(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('{')(input)?;
    
    let mut fields = Vec::new();
    let mut remaining = input;
    
    loop {
        let (rest, _) = ws(remaining).map_err(|_| {
            nom::Err::Error(VerboseError { errors: vec![] })
        })?;
        remaining = rest;

        if remaining.starts_with('}') {
            break;
        }

        // Try parsing a field
        if let Ok((rest, field)) = parse_message_field(remaining) {
            fields.push(field);
            remaining = rest;
            continue;
        }

        // Skip reserved statements
        if let Ok((rest, _)) = parse_reserved(remaining) {
            remaining = rest;
            continue;
        }

        // Skip nested messages (we don't support nested message extraction yet)
        if let Ok((rest, _)) = parse_message(remaining) {
            remaining = rest;
            continue;
        }

        // Skip nested enums
        if let Ok((rest, _)) = parse_nested_enum(remaining) {
            remaining = rest;
            continue;
        }

        // Skip option statements
        if let Ok((rest, _)) = parse_option(remaining) {
            remaining = rest;
            continue;
        }

        // Skip oneof (simplified)
        if remaining.starts_with("oneof") {
            if let Some(end) = remaining.find('}') {
                remaining = &remaining[end + 1..];
                continue;
            }
        }

        // Skip unrecognized content
        if !remaining.is_empty() && !remaining.starts_with('}') {
            let skip = remaining.find(|c: char| c == ';' || c == '}').unwrap_or(1);
            if remaining.chars().nth(skip) == Some(';') {
                remaining = &remaining[skip + 1..];
            } else {
                remaining = &remaining[skip..];
            }
        }
    }

    let (input, _) = ws(remaining)?;
    let (input, _) = char('}')(input)?;

    Ok((
        input,
        StructDef {
            name,
            fields,
            annotations: HashMap::new(),
            comments: Vec::new(),
        },
    ))
}

/// Parse an enum value
fn parse_enum_value(input: &str) -> ParseResult<EnumValue> {
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = int_literal(input)?;
    let (input, _) = parse_field_options(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(';')(input)?;

    Ok((
        input,
        EnumValue {
            name,
            value: Some(value as i32),
            comments: Vec::new(),
        },
    ))
}

/// Parse an enum definition
fn parse_enum(input: &str) -> ParseResult<EnumDef> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("enum")(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('{')(input)?;
    
    let mut values = Vec::new();
    let mut remaining = input;
    
    loop {
        let (rest, _) = ws(remaining).map_err(|_| {
            nom::Err::Error(VerboseError { errors: vec![] })
        })?;
        remaining = rest;

        if remaining.starts_with('}') {
            break;
        }

        // Try parsing an enum value
        if let Ok((rest, value)) = parse_enum_value(remaining) {
            values.push(value);
            remaining = rest;
            continue;
        }

        // Skip option statements
        if let Ok((rest, _)) = parse_option(remaining) {
            remaining = rest;
            continue;
        }

        // Skip reserved statements
        if let Ok((rest, _)) = parse_reserved(remaining) {
            remaining = rest;
            continue;
        }

        // Skip unrecognized content
        if !remaining.is_empty() && !remaining.starts_with('}') {
            let skip = remaining.find(';').map(|i| i + 1).unwrap_or(1);
            remaining = &remaining[skip..];
        }
    }

    let (input, _) = ws(remaining)?;
    let (input, _) = char('}')(input)?;

    Ok((
        input,
        EnumDef {
            name,
            values,
            comments: Vec::new(),
        },
    ))
}

/// Parse RPC method return type
fn parse_rpc_type(input: &str) -> ParseResult<FieldType> {
    let (input, _) = ws(input)?;
    let (input, _stream) = opt(terminated(tag("stream"), multispace1))(input)?;
    let (input, type_name) = fq_identifier(input)?;
    Ok((input, FieldType::Custom(type_name)))
}

/// Parse RPC options
fn parse_rpc_options(input: &str) -> ParseResult<HashMap<String, String>> {
    let (input, _) = ws(input)?;
    let (input, options) = opt(delimited(
        char('{'),
        many0(preceded(
            ws,
            tuple((
                preceded(tag("option"), preceded(ws, fq_identifier)),
                preceded(preceded(ws, char('=')), alt((
                    string_literal,
                    map(fq_identifier, |s| s),
                ))),
                preceded(ws, char(';')),
            )),
        )),
        preceded(ws, char('}')),
    ))(input)?;

    let mut map = HashMap::new();
    if let Some(opts) = options {
        for (key, value, _) in opts {
            map.insert(key, value);
        }
    }
    Ok((input, map))
}

/// Extract HTTP endpoint from RPC annotations (google.api.http)
fn extract_http_endpoint_proto(annotations: &HashMap<String, String>) -> Option<HttpEndpoint> {
    // Check for google.api.http style annotations
    let methods = [
        ("google.api.http.get", HttpMethod::GET),
        ("google.api.http.post", HttpMethod::POST),
        ("google.api.http.put", HttpMethod::PUT),
        ("google.api.http.delete", HttpMethod::DELETE),
        ("google.api.http.patch", HttpMethod::PATCH),
        ("api.get", HttpMethod::GET),
        ("api.post", HttpMethod::POST),
        ("api.put", HttpMethod::PUT),
        ("api.delete", HttpMethod::DELETE),
        ("api.patch", HttpMethod::PATCH),
    ];

    for (key, method) in methods.iter() {
        if let Some(path) = annotations.get(*key) {
            return Some(HttpEndpoint {
                method: method.clone(),
                path: path.clone(),
            });
        }
    }
    None
}

/// Parse an RPC method
fn parse_rpc(input: &str) -> ParseResult<Method> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("rpc")(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('(')(input)?;
    let (input, param_type) = parse_rpc_type(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = tag("returns")(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('(')(input)?;
    let (input, return_type) = parse_rpc_type(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char(')')(input)?;
    let (input, annotations) = parse_rpc_options(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = opt(char(';'))(input)?;

    let http_endpoint = extract_http_endpoint_proto(&annotations);

    Ok((
        input,
        Method {
            name,
            return_type,
            params: vec![MethodParam {
                id: 1,
                name: "request".to_string(),
                param_type,
            }],
            exceptions: Vec::new(),
            oneway: false,
            annotations,
            comments: Vec::new(),
            http_endpoint,
        },
    ))
}

/// Parse a service definition
fn parse_service(input: &str) -> ParseResult<ServiceDef> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("service")(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('{')(input)?;
    
    let mut methods = Vec::new();
    let mut remaining = input;
    
    loop {
        let (rest, _) = ws(remaining).map_err(|_| {
            nom::Err::Error(VerboseError { errors: vec![] })
        })?;
        remaining = rest;

        if remaining.starts_with('}') {
            break;
        }

        // Try parsing an RPC method
        if let Ok((rest, method)) = parse_rpc(remaining) {
            methods.push(method);
            remaining = rest;
            continue;
        }

        // Skip option statements
        if let Ok((rest, _)) = parse_option(remaining) {
            remaining = rest;
            continue;
        }

        // Skip unrecognized content
        if !remaining.is_empty() && !remaining.starts_with('}') {
            let skip = remaining.find(';').map(|i| i + 1).unwrap_or(1);
            remaining = &remaining[skip..];
        }
    }

    let (input, _) = ws(remaining)?;
    let (input, _) = char('}')(input)?;

    Ok((
        input,
        ServiceDef {
            name,
            extends: None,
            methods,
            annotations: HashMap::new(),
            comments: Vec::new(),
        },
    ))
}

/// Parse a complete Proto file
pub fn parse_proto_file(input: &str, file_path: PathBuf) -> Result<ParsedFile, String> {
    let mut parsed = ParsedFile::new(FileType::Proto, file_path);
    let mut remaining = input;

    loop {
        // Skip whitespace and comments
        let (rest, _) = ws(remaining).map_err(|e| format!("Whitespace error: {:?}", e))?;
        remaining = rest;

        if remaining.is_empty() {
            break;
        }

        // Try parsing syntax
        if let Ok((rest, _syntax)) = parse_syntax(remaining) {
            remaining = rest;
            continue;
        }

        // Try parsing package
        if let Ok((rest, pkg)) = parse_package(remaining) {
            parsed.package = Some(pkg);
            remaining = rest;
            continue;
        }

        // Try parsing import
        if let Ok((rest, import)) = parse_import(remaining) {
            parsed.includes.push(import);
            remaining = rest;
            continue;
        }

        // Try parsing option
        if let Ok((rest, (key, value))) = parse_option(remaining) {
            parsed.options.insert(key, value);
            remaining = rest;
            continue;
        }

        // Try parsing message
        if let Ok((rest, msg)) = parse_message(remaining) {
            parsed.structs.push(msg);
            remaining = rest;
            continue;
        }

        // Try parsing enum
        if let Ok((rest, enum_def)) = parse_enum(remaining) {
            parsed.enums.push(enum_def);
            remaining = rest;
            continue;
        }

        // Try parsing service
        if let Ok((rest, service)) = parse_service(remaining) {
            parsed.services.push(service);
            remaining = rest;
            continue;
        }

        // Skip any unrecognized tokens
        if !remaining.is_empty() {
            let skip_count = remaining.find(|c: char| c.is_whitespace() || c == ';')
                .map(|i| if remaining.chars().nth(i) == Some(';') { i + 1 } else { i })
                .unwrap_or(1)
                .max(1);
            remaining = &remaining[skip_count..];
        }
    }

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_syntax() {
        let input = r#"syntax = "proto3";"#;
        let (_, syntax) = parse_syntax(input).unwrap();
        assert_eq!(syntax, "proto3");
    }

    #[test]
    fn test_parse_package() {
        let input = "package example.user;";
        let (_, pkg) = parse_package(input).unwrap();
        assert_eq!(pkg, "example.user");
    }

    #[test]
    fn test_parse_message() {
        let input = r#"
        message User {
            int64 id = 1;
            string name = 2;
            repeated string tags = 3;
        }
        "#;
        let (_, msg) = parse_message(input).unwrap();
        assert_eq!(msg.name, "User");
        assert_eq!(msg.fields.len(), 3);
    }

    #[test]
    fn test_parse_service() {
        let input = r#"
        service UserService {
            rpc GetUser(GetUserRequest) returns (User);
            rpc CreateUser(CreateUserRequest) returns (User);
        }
        "#;
        let (_, service) = parse_service(input).unwrap();
        assert_eq!(service.name, "UserService");
        assert_eq!(service.methods.len(), 2);
    }
}
