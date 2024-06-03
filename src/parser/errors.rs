use crate::lexer::Coord;

pub fn pretty_errors<'input>(
    src: &'input str,
    errors: Vec<lalrpop_util::ErrorRecovery<Coord, crate::lexer::Token<'_>, &str>>,
) -> String {
    let mut pretty = String::new();

    for error in errors {
        match error.error {
            lalrpop_util::ParseError::InvalidToken { location: _ } => todo!(),
            lalrpop_util::ParseError::UnrecognizedEof { location, expected } => {
                let start = location;
                let end = location;
                let start_line = start.line;
                let end_line = end.line;
                let line = &src[start_line..end_line];
                let start_col = start.col;
                let end_col = end.col;
                pretty.push_str(&format!(
                    "error: unexpected end of file, expected one of {:?}\n",
                    expected
                ));
                pretty.push_str(&line);
                pretty.push_str("\n");
                pretty.push_str(&" ".repeat(start_col));
                pretty.push_str(&"^".repeat(end_col - start_col));
            }
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
                // expected is double quoted, so we need to remove the quotes
                // which ends up printing like   "\"(\""
                let expected: Vec<&str> = expected.iter().map(|x| &x[1..x.len() - 1]).collect();
                pretty.push_str(&format!(
                    "error: unexpected token {:?}, expected one of {:?}\n",
                    token.1, expected
                ));
                pretty.push_str(&line);
                pretty.push_str("\n");
                pretty.push_str(&" ".repeat(start_col));
                pretty.push_str(&"^".repeat(end_col - start_col));
            }
            lalrpop_util::ParseError::ExtraToken { token: _ } => todo!(),
            lalrpop_util::ParseError::User { error: _ } => todo!(),
        };
    }
    // pretty.push_str(&src[last_end..]);
    pretty
}
