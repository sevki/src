use crate::{
    compiler::text::{self, Position, SourceMap, SourceProgram, Span, Spanned},
    Db,
};
use okstd::prelude::*;
use std::ops::Range;

#[salsa::tracked]
pub fn get_symbol(db: &dyn Db, src: text::SourceProgram, pos: text::Position) -> Option<Spanned> {
    let spans = text::to_spans(db, src);
    let tokens = spans.tokens(db);
    let token = tokens.iter().find(|x| {
        debug!("line: {} == {}", x.pos(db).line(db), pos.line(db));
        let start = x.pos(db).column(db) - x.span(db).span(db).len() + 1;
        let end = x.pos(db).column(db);
        debug!("start: {} <= {} <= {}", start, pos.column(db), end);
        debug!("column: {} == {}", x.pos(db).column(db), pos.column(db));
        x.pos(db).line(db) == pos.line(db) && start <= pos.column(db) && pos.column(db) <= end
    });
    token.cloned()
}

#[salsa::input]
pub struct Url {
    #[id]
    pub url: String,
}

#[salsa::tracked]
pub fn add_file(db: &dyn Db, url: Url, text: String) -> SourceProgram {
    SourceProgram::new(db, url.url(db), text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::db::Database;

    #[okstd::log(off)]
    #[okstd::test]
    fn test_get_symbol() {
        let db = &Database::default();

        let src = text::SourceProgram::new(
            db,
            "inmemory://test".to_string(),
            r#"fn main() {}"#.to_string(),
        );
        let symb = get_symbol(db, src, Position::new(db, 0, 5));
        assert!(symb.is_some());
        let got = span_text(db, symb.unwrap());
        assert_eq!(got, "main");

        let symb = get_symbol(db, src, Position::new(db, 0, 8));
        assert!(symb.is_some());
        let got = span_text(db, symb.unwrap());
        assert_eq!(got, "(");
    }

    #[okstd::log(off)]
    #[okstd::test]
    fn test_add_file() {
        let db = &Database::default();
        let url = Url::new(db, "inmemory://test".to_string());
        let src = add_file(
            db,
            url,
            r#"use { host } from std

effect Make: async + throws + execs + reads + writes {
    catch() [throws]
    await<T>(f: Future<T>) [async, throws] -> T
    exec(arg0: string, args: stringvec) [Make] -> i32
}

struct Local {
    host: host
}

impl Make for Local {
    fn catch(self) [throws] {
    }
    fn await<T>(f: Future<T>) [async, trhows] -> T {
        yield()
    }
    fn exec(self, arg0: string, args: vec<string>) [Vm] -> i32 {
        self.host.read("jobserver")
        if self.host.exec(arg0, args) {
            raise(1)
        }
    }
}"#
            .to_string(),
        );

        let symb = get_symbol(db, src, Position::new(db, 21, 16));
        assert!(symb.is_some());
        let got = span_text(db, symb.unwrap());
        assert_eq!(got, "raise");
    }
}

#[salsa::tracked]
pub fn span_text(db: &dyn Db, span: Spanned) -> String {
    span.src(db).text(db)[span.span(db).span(db)].to_string()
}
