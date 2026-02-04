//! Thrift IDL parser implementation

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
    // Handle comments
    let (input, _) = many0(alt((
        preceded(tag("//"), take_until("\n")),
        preceded(tag("#"), take_until("\n")),
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

/// Parse a string literal
fn string_literal(input: &str) -> ParseResult<String> {
    let (input, _) = ws(input)?;
    let (input, s) = alt((
        delimited(
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
        ),
        delimited(
            char('\''),
            map(
                opt(escaped(
                    take_while1(|c| c != '\'' && c != '\\'),
                    '\\',
                    one_of("'\\nrt"),
                )),
                |o| o.unwrap_or(""),
            ),
            char('\''),
        ),
    ))(input)?;
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

/// Parse a namespace declaration
fn parse_namespace(input: &str) -> ParseResult<(String, String)> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("namespace")(input)?;
    let (input, lang) = identifier(input)?;
    let (input, ns) = alt((identifier, string_literal))(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = opt(alt((char(';'), char(','))))(input)?;
    Ok((input, (lang, ns)))
}

/// Parse an include statement
fn parse_include(input: &str) -> ParseResult<IncludeDef> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("include")(input)?;
    let (input, path) = string_literal(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = opt(alt((char(';'), char(','))))(input)?;
    Ok((input, IncludeDef { path, alias: None }))
}

/// Parse a field type
fn parse_field_type(input: &str) -> ParseResult<FieldType> {
    let (input, _) = ws(input)?;
    alt((
        // Container types
        map(
            preceded(
                tag("list"),
                delimited(
                    preceded(ws, char('<')),
                    parse_field_type,
                    preceded(ws, char('>')),
                ),
            ),
            |t| FieldType::List(Box::new(t)),
        ),
        map(
            preceded(
                tag("set"),
                delimited(
                    preceded(ws, char('<')),
                    parse_field_type,
                    preceded(ws, char('>')),
                ),
            ),
            |t| FieldType::Set(Box::new(t)),
        ),
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
        value(FieldType::Byte, alt((tag("byte"), tag("i8")))),
        value(FieldType::I16, tag("i16")),
        value(FieldType::I32, tag("i32")),
        value(FieldType::I64, tag("i64")),
        value(FieldType::Double, tag("double")),
        value(FieldType::Float, tag("float")),
        value(FieldType::String, tag("string")),
        value(FieldType::Binary, tag("binary")),
        value(FieldType::Void, tag("void")),
        // Custom types (must be last)
        map(
            recognize(pair(
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_"), tag(".")))),
            )),
            |s: &str| FieldType::Custom(s.to_string()),
        ),
    ))(input)
}

/// Parse annotations (e.g., (api.get = "/path"))
fn parse_annotations(input: &str) -> ParseResult<HashMap<String, String>> {
    let (input, _) = ws(input)?;
    let (input, annotations) = opt(delimited(
        char('('),
        separated_list0(
            preceded(ws, char(',')),
            tuple((
                preceded(ws, recognize(pair(
                    alt((alpha1, tag("_"))),
                    many0(alt((alphanumeric1, tag("_"), tag(".")))),
                ))),
                preceded(preceded(ws, char('=')), alt((string_literal, map(identifier, |s| s)))),
            )),
        ),
        preceded(ws, char(')')),
    ))(input)?;

    let mut map = HashMap::new();
    if let Some(anns) = annotations {
        for (key, value) in anns {
            map.insert(key.to_string(), value);
        }
    }
    Ok((input, map))
}

/// Parse a struct field
fn parse_field(input: &str) -> ParseResult<Field> {
    let (input, _) = ws(input)?;
    let (input, id) = opt(terminated(int_literal, preceded(ws, char(':'))))(input)?;
    let (input, _) = ws(input)?;
    let (input, req) = opt(alt((
        value(true, tag("required")),
        value(false, tag("optional")),
    )))(input)?;
    let (input, field_type) = parse_field_type(input)?;
    let (input, name) = identifier(input)?;
    let (input, default_value) = opt(preceded(
        preceded(ws, char('=')),
        alt((
            map(string_literal, |s| s),
            map(int_literal, |i| i.to_string()),
            identifier,
        )),
    ))(input)?;
    let (input, annotations) = parse_annotations(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = opt(alt((char(';'), char(','))))(input)?;

    Ok((
        input,
        Field {
            id: id.unwrap_or(0) as i32,
            name,
            field_type,
            required: req.unwrap_or(false),
            optional: !req.unwrap_or(true),
            default_value,
            annotations,
            comments: Vec::new(),
        },
    ))
}

/// Parse a struct definition
fn parse_struct(input: &str) -> ParseResult<StructDef> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("struct")(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('{')(input)?;
    let (input, fields) = many0(parse_field)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('}')(input)?;
    let (input, annotations) = parse_annotations(input)?;

    Ok((
        input,
        StructDef {
            name,
            fields,
            annotations,
            comments: Vec::new(),
        },
    ))
}

/// Parse an enum value
fn parse_enum_value(input: &str) -> ParseResult<EnumValue> {
    let (input, _) = ws(input)?;
    let (input, name) = identifier(input)?;
    let (input, value) = opt(preceded(preceded(ws, char('=')), int_literal))(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = opt(alt((char(';'), char(','))))(input)?;

    Ok((
        input,
        EnumValue {
            name,
            value: value.map(|v| v as i32),
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
    let (input, values) = many0(parse_enum_value)(input)?;
    let (input, _) = ws(input)?;
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

/// Parse a method parameter
fn parse_method_param(input: &str) -> ParseResult<MethodParam> {
    let (input, _) = ws(input)?;
    let (input, id) = opt(terminated(int_literal, preceded(ws, char(':'))))(input)?;
    let (input, param_type) = parse_field_type(input)?;
    let (input, name) = identifier(input)?;

    Ok((
        input,
        MethodParam {
            id: id.unwrap_or(0) as i32,
            name,
            param_type,
        },
    ))
}

/// Parse method exceptions
fn parse_method_exceptions(input: &str) -> ParseResult<Vec<ExceptionDef>> {
    let (input, _) = ws(input)?;
    let (input, exceptions) = opt(preceded(
        tag("throws"),
        delimited(
            preceded(ws, char('(')),
            separated_list0(
                preceded(ws, char(',')),
                map(
                    tuple((
                        opt(terminated(int_literal, preceded(ws, char(':')))),
                        parse_field_type,
                        identifier,
                    )),
                    |(id, exception_type, name)| ExceptionDef {
                        id: id.unwrap_or(0) as i32,
                        name,
                        exception_type,
                    },
                ),
            ),
            preceded(ws, char(')')),
        ),
    ))(input)?;

    Ok((input, exceptions.unwrap_or_default()))
}

/// Extract HTTP endpoint from annotations
fn extract_http_endpoint(annotations: &HashMap<String, String>) -> Option<HttpEndpoint> {
    let methods = [
        ("api.get", HttpMethod::GET),
        ("api.post", HttpMethod::POST),
        ("api.put", HttpMethod::PUT),
        ("api.delete", HttpMethod::DELETE),
        ("api.patch", HttpMethod::PATCH),
        ("api.head", HttpMethod::HEAD),
        ("api.options", HttpMethod::OPTIONS),
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

/// Parse a service method
fn parse_method(input: &str) -> ParseResult<Method> {
    let (input, _) = ws(input)?;
    let (input, oneway) = opt(terminated(tag("oneway"), multispace1))(input)?;
    let (input, return_type) = parse_field_type(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, params) = delimited(
        char('('),
        separated_list0(preceded(ws, char(',')), parse_method_param),
        preceded(ws, char(')')),
    )(input)?;
    let (input, exceptions) = parse_method_exceptions(input)?;
    let (input, annotations) = parse_annotations(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = opt(alt((char(';'), char(','))))(input)?;

    let http_endpoint = extract_http_endpoint(&annotations);

    Ok((
        input,
        Method {
            name,
            return_type,
            params,
            exceptions,
            oneway: oneway.is_some(),
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
    let (input, extends) = opt(preceded(tag("extends"), identifier))(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('{')(input)?;
    let (input, methods) = many0(parse_method)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('}')(input)?;
    let (input, annotations) = parse_annotations(input)?;

    Ok((
        input,
        ServiceDef {
            name,
            extends,
            methods,
            annotations,
            comments: Vec::new(),
        },
    ))
}

/// Parse a const definition
fn parse_const(input: &str) -> ParseResult<ConstDef> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("const")(input)?;
    let (input, const_type) = parse_field_type(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('=')(input)?;
    let (input, value) = alt((
        string_literal,
        map(int_literal, |i| i.to_string()),
        identifier,
    ))(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = opt(alt((char(';'), char(','))))(input)?;

    Ok((
        input,
        ConstDef {
            name,
            const_type,
            value,
            comments: Vec::new(),
        },
    ))
}

/// Parse a typedef definition
fn parse_typedef(input: &str) -> ParseResult<TypedefDef> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("typedef")(input)?;
    let (input, original_type) = parse_field_type(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = opt(alt((char(';'), char(','))))(input)?;

    Ok((
        input,
        TypedefDef {
            name,
            original_type,
            comments: Vec::new(),
        },
    ))
}

/// Parse an exception struct
fn parse_exception_struct(input: &str) -> ParseResult<StructDef> {
    let (input, _) = ws(input)?;
    let (input, _) = tag("exception")(input)?;
    let (input, name) = identifier(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('{')(input)?;
    let (input, fields) = many0(parse_field)(input)?;
    let (input, _) = ws(input)?;
    let (input, _) = char('}')(input)?;
    let (input, annotations) = parse_annotations(input)?;

    Ok((
        input,
        StructDef {
            name,
            fields,
            annotations,
            comments: Vec::new(),
        },
    ))
}

/// Parse a complete Thrift file
pub fn parse_thrift_file(input: &str, file_path: PathBuf) -> Result<ParsedFile, String> {
    let mut parsed = ParsedFile::new(FileType::Thrift, file_path);
    let mut remaining = input;

    loop {
        // Skip whitespace and comments
        let (rest, _) = ws(remaining).map_err(|e| format!("Whitespace error: {:?}", e))?;
        remaining = rest;

        if remaining.is_empty() {
            break;
        }

        // Try parsing each type of definition
        if let Ok((rest, (lang, ns))) = parse_namespace(remaining) {
            parsed.namespace.insert(lang, ns);
            remaining = rest;
            continue;
        }

        if let Ok((rest, include)) = parse_include(remaining) {
            parsed.includes.push(include);
            remaining = rest;
            continue;
        }

        if let Ok((rest, struct_def)) = parse_struct(remaining) {
            parsed.structs.push(struct_def);
            remaining = rest;
            continue;
        }

        if let Ok((rest, struct_def)) = parse_exception_struct(remaining) {
            parsed.structs.push(struct_def);
            remaining = rest;
            continue;
        }

        if let Ok((rest, enum_def)) = parse_enum(remaining) {
            parsed.enums.push(enum_def);
            remaining = rest;
            continue;
        }

        if let Ok((rest, service)) = parse_service(remaining) {
            parsed.services.push(service);
            remaining = rest;
            continue;
        }

        if let Ok((rest, const_def)) = parse_const(remaining) {
            parsed.constants.push(const_def);
            remaining = rest;
            continue;
        }

        if let Ok((rest, typedef)) = parse_typedef(remaining) {
            parsed.typedefs.push(typedef);
            remaining = rest;
            continue;
        }

        // Skip any unrecognized tokens
        if !remaining.is_empty() {
            let skip_count = remaining.find(|c: char| c.is_whitespace()).unwrap_or(1).max(1);
            remaining = &remaining[skip_count..];
        }
    }

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_namespace() {
        let input = "namespace go example.user";
        let (_, (lang, ns)) = parse_namespace(input).unwrap();
        assert_eq!(lang, "go");
        assert_eq!(ns, "example.user");
    }

    #[test]
    fn test_parse_include() {
        let input = r#"include "common.thrift""#;
        let (_, include) = parse_include(input).unwrap();
        assert_eq!(include.path, "common.thrift");
    }

    #[test]
    fn test_parse_struct() {
        let input = r#"
        struct User {
            1: required i64 id
            2: optional string name
            3: bool active = true
        }
        "#;
        let (_, struct_def) = parse_struct(input).unwrap();
        assert_eq!(struct_def.name, "User");
        assert_eq!(struct_def.fields.len(), 3);
    }

    #[test]
    fn test_parse_service() {
        let input = r#"
        service UserService {
            User getUser(1: i64 id) (api.get = "/user/:id")
            void createUser(1: User user) (api.post = "/user")
        }
        "#;
        let (_, service) = parse_service(input).unwrap();
        assert_eq!(service.name, "UserService");
        assert_eq!(service.methods.len(), 2);
        assert!(service.methods[0].http_endpoint.is_some());
    }
}
