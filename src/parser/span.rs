pub use crate::{lexer::Location, Db};
use hashbrown::HashMap;
use okstd::prelude::*;
use std::{fmt::Display, ops::Range};

use super::ast::Node;

#[derive(PartialEq, Debug, Clone, Eq, PartialOrd)]
pub struct Spanned<T>(pub Location, pub T, pub Location);

impl<T> Spanned<T> {
    pub fn span(&self) -> Range<Location> {
        self.0..self.2
    }
    pub fn span_size(&self) -> usize {
        self.span().end.offset - self.span().start.offset
    }
}

trait GetSelf<T> {
    fn into(self) -> T;
}

impl<T> GetSelf<T> for Spanned<T> {
    fn into(self) -> T {
        self.1
    }
}

impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.1)
    }
}

impl<T> Spanned<T> {
    pub fn overlap(&self, other: &ByteOrLineColOrCoord) -> bool {
        match other {
            ByteOrLineColOrCoord::Byte(byte) => self.0.offset <= *byte && *byte <= self.2.offset,
            ByteOrLineColOrCoord::LineCol(line, col) => {
                debug!("line: {}, col: {}", line, col);
                (*line == self.0.line || *line == self.2.line) && {
                    //start and end are on the same line
                    if self.0.line == self.2.line {
                        self.0.col <= *col && *col <= self.2.col
                    }
                    // if start and end are on different lines
                    // make sure the line is between the start and end
                    else {
                        if self.0.line <= *line && *line <= self.2.line {
                            return self.0.col <= *col && *col <= self.2.col;
                        }
                        // if the line is the same as the start
                        false
                    }
                }
            }
            ByteOrLineColOrCoord::Coord(coord) => {
                self.0.offset <= coord.offset && coord.offset <= self.2.offset
            }
        }
    }
}

#[salsa::input]
pub struct ByteOrLineColOrCoordInterned {
    #[return_ref]
    pub pos: ByteOrLineColOrCoord,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ByteOrLineColOrCoord {
    Byte(usize),
    LineCol(usize, usize),
    Coord(Location),
}

pub trait HasChildSpans
where
    Self: Send + Sync + Sized + Spanning,
{
    fn children(&self) -> impl IntoIterator<Item = Self>;
}

pub trait Spanning: Send + Sync + Sized {
    fn span(&self) -> Range<Location>;
    fn byte_range(&self) -> Range<usize> {
        self.span().start.offset..self.span().end.offset
    }
    fn line_col_range(&self) -> Range<(usize, usize)> {
        (self.span().start.line, self.span().start.col)..(self.span().end.line, self.span().end.col)
    }
    fn overlap(&self, other: &ByteOrLineColOrCoord) -> bool {
        match other {
            ByteOrLineColOrCoord::Byte(offset) => self.byte_range().contains(offset),
            ByteOrLineColOrCoord::LineCol(line, col) => {
                self.line_col_range().contains(&(*line, *col))
            }
            ByteOrLineColOrCoord::Coord(coord) => self.span().contains(coord),
        }
    }
}

impl<T: Sized + Send + Sync> Spanning for Spanned<T> {
    fn span(&self) -> Range<Location> {
        self.0..self.2
    }
}

impl Spanning for &Range<Location> {
    fn span(&self) -> Range<Location> {
        #![allow(suspicious_double_ref_op)]
        self.clone().clone()
    }
}

// span macro
#[macro_export]
macro_rules! span {
    ($coord:expr, $expr:expr) => {
        Spanned($coord, $expr, $coord)
    };
    ($start:expr, $expr:expr, $end:expr) => {
        Spanned($start, $expr, $end)
    };
}

// SourceMap, is the only way to construct a Spanned<T> object
// this allows us to create a map of T to Range<Coord>,
// which allows us to find T given some Coord
// this optimizes the search for the symbol at a given Coord
// which is useful for hover and go to definition
#[salsa::tracked]
pub struct SourceMap {
    #[return_ref]
    tokens: HashMap<Range<Location>, Node>,
}
