use std::ops::Range;

use crate::Db;

use super::text::SourceProgram;


pub struct Errors<'a>(Vec<lalrpop_util::ErrorRecovery<usize, crate::lexer::Token<'a>, &'a str>>);

impl<'a> From<Vec<lalrpop_util::ErrorRecovery<usize, crate::lexer::Token<'a>, &'a str>>>
    for Errors<'a>
{
    fn from(
        errors: Vec<lalrpop_util::ErrorRecovery<usize, crate::lexer::Token<'a>, &'a str>>,
    ) -> Self {
        Self(errors)
    }
}

impl<'a> IntoIterator for Errors<'a> {
    type Item = Range<usize>;

    type IntoIter = <Vec<std::ops::Range<usize>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|error| match error.error {
                lalrpop_util::ParseError::InvalidToken { location } => location..location,
                lalrpop_util::ParseError::UnrecognizedEof { location, expected } => {
                    location..location
                }
                lalrpop_util::ParseError::UnrecognizedToken { token, expected } => token.0..token.2,
                lalrpop_util::ParseError::ExtraToken { token } => token.0..token.2,
                lalrpop_util::ParseError::User { error } => todo!(),
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}


fn handle_errors(errors: Vec<lalrpop_util::ErrorRecovery<usize, crate::lexer::Token, &str>>, src: &str) -> String {
    let mut pretty = String::new();
    let mut last_end = 0;
    
    for error in errors {
        match error.error {
            lalrpop_util::ParseError::InvalidToken { location } => todo!(),
            lalrpop_util::ParseError::UnrecognizedEof { location, expected } => todo!(),
            lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                // find the line and column of the start and end tokens,
                // and print the line with a caret pointing to the error
                let start = token.0;
                let end = token.2;
                let start_line = src[..start].rfind('\n').map_or(0, |i| i + 1);
                let end_line = src[end..].find('\n').map_or(src.len(), |i| end + i);
                let line = &src[start_line..end_line];
                let start_col = start - start_line;
                let end_col = end - start_line;
                // pretty.push_str(&src[last_end..start]);
                pretty.push_str(&format!("error: unexpected token {:?}, expected one of {:?}\n", token.1, expected));
                pretty.push_str(&line);
                pretty.push_str("\n");
                pretty.push_str(&" ".repeat(start_col));
                pretty.push_str(&"^".repeat(end_col - start_col));
                last_end = end;
            },
            lalrpop_util::ParseError::ExtraToken { token } => todo!(),
            lalrpop_util::ParseError::User { error } => todo!(),
        };
        
    }
    // pretty.push_str(&src[last_end..]);
    pretty
}