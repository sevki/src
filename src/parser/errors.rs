pub fn pretty_errors<'input>(src: &'input str, errors: Vec<lalrpop_util::ErrorRecovery<usize, crate::lexer::Token, &str>>) -> String {
    let mut pretty = String::new();
    let mut last_end = 0;
    for error in errors {
        match error.error {
            lalrpop_util::ParseError::InvalidToken { location: _ } => todo!(),
            lalrpop_util::ParseError::UnrecognizedEof { location: _, expected: _ } => todo!(),
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
            lalrpop_util::ParseError::ExtraToken { token: _ } => todo!(),
            lalrpop_util::ParseError::User { error: _ } => todo!(),
        };
        
    }
    // pretty.push_str(&src[last_end..]);
    pretty
}
