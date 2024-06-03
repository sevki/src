use std::ops::Range;

use crate::lexer::Coord;

pub struct Errors<'a>(Vec<lalrpop_util::ErrorRecovery<Coord, crate::lexer::Token<'a>, &'a str>>);

impl<'a> From<Vec<lalrpop_util::ErrorRecovery<Coord, crate::lexer::Token<'a>, &'a str>>>
    for Errors<'a>
{
    fn from(
        value: Vec<lalrpop_util::ErrorRecovery<Coord, crate::lexer::Token<'a>, &'a str>>,
    ) -> Self {
        Self(value)
    }
}

impl<'a> IntoIterator for Errors<'a> {
    type Item = Range<Coord>;

    type IntoIter = <Vec<std::ops::Range<Coord>> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|error| match error.error {
                lalrpop_util::ParseError::InvalidToken { location } => location..location,
                lalrpop_util::ParseError::UnrecognizedEof {
                    location,
                    expected: _,
                } => location..location,
                lalrpop_util::ParseError::UnrecognizedToken { token, expected: _ } => {
                    token.0..token.2
                }
                lalrpop_util::ParseError::ExtraToken { token } => token.0..token.2,
                lalrpop_util::ParseError::User { error: _ } => todo!(),
            })
            .collect::<Vec<_>>()
            .into_iter()
    }
}

fn handle_errors(
    errors: Vec<lalrpop_util::ErrorRecovery<Coord, crate::lexer::Token, &str>>,
    src: &str,
) -> String {
    let mut pretty = String::new();
    let mut last_end = Coord::default();

    for error in errors {
        match error.error {
            lalrpop_util::ParseError::InvalidToken { location: _ } => todo!(),
            lalrpop_util::ParseError::UnrecognizedEof {
                location: _,
                expected: _,
            } => todo!(),
            lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                // find the line and column of the start and end tokens,
                // and print the line with a caret pointing to the error
                let start = token.0;
                let end = token.2;
                let start_line = start.line;
                let end_line = end.line;
                let line = &src[start_line..end_line];
                let start_col = start.col;
                let end_col = end.col;
                // pretty.push_str(&src[last_end..start]);
                pretty.push_str(&format!(
                    "error: unexpected token {:?}, expected one of {:?}\n",
                    token.1, expected
                ));
                pretty.push_str(&line);
                pretty.push_str("\n");
                pretty.push_str(&" ".repeat(start_col));
                pretty.push_str(&"^".repeat(end_col - start_col));
                last_end = end;
            }
            lalrpop_util::ParseError::ExtraToken { token: _ } => todo!(),
            lalrpop_util::ParseError::User { error: _ } => todo!(),
        };
    }
    // pretty.push_str(&src[last_end..]);
    pretty
}
