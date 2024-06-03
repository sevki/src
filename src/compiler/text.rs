use std::ops::Range;

use crate::{lexer::Token, Db};
use bitflags::bitflags;

/// Represents the source program text.
#[salsa::input]
pub struct SourceProgram {
    #[id]
    pub url: String,

    #[return_ref]
    pub text: String,
}

/// Represents a spanned piece of code.
#[salsa::interned]
pub struct Spanned {
    /// The span of the code.
    #[return_ref]
    pub span: Span,

    /// The source program associated with the code.
    #[return_ref]
    pub src: SourceProgram,

    /// The position of the code in the source program.
    #[return_ref]
    pub pos: Position,
}

/// Represents a span of text.
#[salsa::interned]
pub struct Span {
    /// The range of the span in the source program text.
    pub span: Range<usize>,
}
/// Represents a position in the source code.
#[salsa::interned]
pub struct Position {
    /// The line number of the position.
    pub line: usize,

    /// The column number of the position.
    pub column: usize,
}

/// Represents the source map of the program.
#[salsa::tracked]
pub struct SourceMap {
    #[return_ref]
    pub tokens: Vec<Spanned>,
}

#[salsa::tracked]
pub fn calculate_line_lengths(db: &dyn Db, src: SourceProgram) -> Vec<usize> {
    src.text(db).lines().map(|x| x.len()).collect()
}

// spanoverlap is a bitflag that is used to determine how two spans overlap
// it is used to determine if a token is within a line
// it is rare a token will span multiple lines but it is possible
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SpanOverlap: u8 {
        const NONE = 0b0000;
        const START = 0b0001;
        const END = 0b0010;
        const BOTH = 0b0011;
    }
}

#[inline]
pub fn cmp_range<T: Ord>(a: &Range<T>, b: &Range<T>) -> SpanOverlap {
    let mut overlap = SpanOverlap::NONE;
    if a.contains(&b.start) {
        overlap |= SpanOverlap::START;
    }
    if a.contains(&b.end) {
        overlap |= SpanOverlap::END;
    }
    overlap
}

#[salsa::tracked]
pub fn to_spans(db: &dyn Db, src: SourceProgram) -> SourceMap {
    let mut spans = vec![];

    let lexer = crate::lexer::Lexer::new(src.text(db), 0);
    // this is sort of a zip~ish operation.
    // we have to arrays that we are iterating over. One is build cheaply, the line lengths
    // and the other is built expensively, the lexer.
    // Lexer tokens have a start and end position, and we want to map these to the line lengths
    // first we iterate over the lexer tokens
    for token in lexer {
        match token.node {
            crate::lexer::Token::Eof => break,
            crate::lexer::Token::NewLine => continue,
            _ => {}
        }
        spans.push(Spanned::new(
            db,
            Span::new(db, token.start..token.end),
            src,
            Position::new(db, token.pos.line, token.pos.col),
        ));
    }
    SourceMap::new(db, spans)
}
use okstd::prelude::*;
use syn::token;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::db::Database;
    use okstd::prelude::*;
    #[okstd::log(off)]
    #[okstd::test]
    fn test_to_spans() {
        let db = Database::default();
        let src = SourceProgram::new(
            &db,
            "inmemory://test".to_string(),
            r#"fn main() {}"#.to_string(),
        );
        let spans = to_spans(&db, src);
        let tokens = spans.tokens(&db);
        for token in tokens.iter() {
            debug!("line {:?}", token.pos(&db).line(&db));
            debug!("column {:?}", token.pos(&db).column(&db));
        }
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].pos(&db).line(&db), 0);
    }
    #[okstd::log(trace)]
    #[okstd::test]
    fn test_to_spans_multiline() {
        let db = Database::default();
        let src = SourceProgram::new(
            &db,
            "inmemory://test".to_string(),
            r#"fn main() {
    let x = 1
}"#
            .to_string(),
        );
        let spans = to_spans(&db, src);
        let tokens = spans.tokens(&db);
        for token in tokens.iter() {
            debug!("line {:?}", token.pos(&db).line(&db));
            debug!("column {:?}", token.pos(&db).column(&db));
        }
        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].pos(&db).line(&db), 0);
    }
}
