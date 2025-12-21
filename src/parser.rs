use regex::Regex;

/// Parses IDL content and extracts include directives
pub struct IdlParser {
    include_regex: Regex,
}

impl IdlParser {
    pub fn new() -> Self {
        // Matches common IDL include patterns:
        // #include "filename.idl"
        // #include <filename.idl>
        // import "filename.idl"
        let include_regex = Regex::new(
            r#"(?m)^\s*(?:#include|import)\s*[<"]([^>"]+)[>"]"#
        ).expect("Failed to compile regex");
        
        Self { include_regex }
    }

    /// Extract all include file paths from IDL content
    pub fn extract_includes(&self, content: &str) -> Vec<String> {
        let mut includes = Vec::new();
        
        for cap in self.include_regex.captures_iter(content) {
            if let Some(path) = cap.get(1) {
                includes.push(path.as_str().to_string());
            }
        }
        
        includes
    }

}

impl Default for IdlParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_includes() {
        let parser = IdlParser::new();
        
        let idl_content = r#"
        #include "base.idl"
        #include <system.idl>
        import "another.idl"
        
        // This should not match: include "commented.idl"
        
        interface MyInterface {
            void myMethod();
        };
        "#;
        
        let includes = parser.extract_includes(idl_content);
        assert_eq!(includes.len(), 3);
        assert!(includes.contains(&"base.idl".to_string()));
        assert!(includes.contains(&"system.idl".to_string()));
        assert!(includes.contains(&"another.idl".to_string()));
    }

    #[test]
    fn test_no_includes() {
        let parser = IdlParser::new();
        let idl_content = r#"
        interface MyInterface {
            void myMethod();
        };
        "#;
        
        let includes = parser.extract_includes(idl_content);
        assert_eq!(includes.len(), 0);
    }
}
