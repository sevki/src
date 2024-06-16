

use syn::spanned::Spanned as _;



#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedChar(char),
    UnterminatedString,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedChar(c) => write!(f, "unexpected character: '{}'", c),
            ParseError::UnterminatedString => write!(f, "unterminated string"),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn apply_string_escapes(content: &str) -> std::borrow::Cow<str> {
    let s = syn::LitStr::new(content, content.span());
    s.token().to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_string_escapes() {
        assert_eq!(apply_string_escapes(r#"hello"#), "\"hello\"");
    }
    #[test]
    fn test_apply_string_escapes_with_escaped_quote() {
        assert_eq!(apply_string_escapes(r#"hello" world"#), r#""hello\" world""#);
    }

    #[test]
    fn test_apply_string_escapes_with_escaped_backslash() {
        assert_eq!(apply_string_escapes(r#"hello\" world"#), r#""hello\\\" world""#);
    }
}

