use std::ops::Range;

use bitflags::bitflags;
use crate::Db;

/// Represents the source program text.
#[salsa::input]
pub struct SourceProgram {
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
    pub span: (usize, usize),
}

/// Represents a position in the source code.
#[salsa::interned]
pub struct Position {
    /// The line number of the position.
    l: usize,

    /// The column number of the position.
    c: usize,
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
fn cmp_range<T: Ord>(a: &Range<T>, b: &Range<T>) -> SpanOverlap {
    let mut overlap = SpanOverlap::NONE;
    if a.contains(&b.start) {
        overlap |= SpanOverlap::START;
    }
    if a.contains(&b.end) {
        overlap |= SpanOverlap::END;
    }
    overlap
}


/// todo(sevki): split this into two functions
#[salsa::tracked]
pub fn to_spans(db: &dyn Db, src: SourceProgram) -> SourceMap {
    let line_lengths: Vec<Range<usize>> = calculate_line_lengths(db, src)
        .into_iter()
        .scan(0, |acc, x| {
            let range = *acc..*acc + x;
            *acc += x;
            Some(range)
        })
        .collect();

    // reverse the line lengths and make it peakable essentially
    // turinging it into a stack
    let mut line_lengths = line_lengths.into_iter().enumerate().rev().peekable();

    let mut spans = vec![];

    let lexer = crate::lexer::Lexer::new(src.text(db), 0);
    // this is sort of a zip~ish operation.
    // we have to arrays that we are iterating over. One is build cheaply, the line lengths
    // and the other is built expensively, the lexer.
    // Lexer tokens have a start and end position, and we want to map these to the line lengths
    // first we iterate over the lexer tokens
    for token in lexer {
        let _size = token.end - token.start;
        // then we peek at the first line
        let mut start: Option<(usize, usize)> = None;
        loop {
            if let Some((line_no, span)) = line_lengths.clone().peek() {
                // if the token is within the line
                let overlap = cmp_range(&span, &(token.start..token.end));
                if overlap == SpanOverlap::NONE && start.is_none() {
                    // if the token is not within the line
                    line_lengths.next();
                }
                if overlap == SpanOverlap::START || overlap == SpanOverlap::BOTH {
                    // if the token is within the line
                    start = Some((*line_no, span.start));
                    // we do not need to iterate more.
                    break;
                }
            }
        }

        if start.is_none() {
            // if the token is not within the line
            break;
        }
        let start = start.unwrap();
        let leading_chars = src.text(db).get(start.1..token.start);
        let column = leading_chars.map(|x| x.chars().count()).unwrap_or(0);
        /*
        ```text
           1,1   7
           |     |
           # Intro
        8  lorem ipsum dolor sit amet
                 │
                 13 byte start
                 6th column, 2nd line
        ```
        */
        spans.push(Spanned::new(
            db,
            Span::new(db, (token.start, token.end)),
            src,
            Position::new(db, start.0, column),
        ));
    }
    SourceMap::new(db, spans)
}
