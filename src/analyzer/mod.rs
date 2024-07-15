pub mod db;

use crate::{
    compiler::text::{self, SourceProgram},
    parser::{
        ast::{self, Node},
        span::{self, ByteOrLineColOrCoord, Spanned},
    },
    Db,
};

use okstd::prelude::*;

#[salsa::input]
pub struct SyntaxTree {
    exprs: Vec<Spanned<ast::Node>>,
}

impl SyntaxTree {
    fn find_span(self, db: &dyn Db, pos: &ByteOrLineColOrCoord) -> Option<Spanned<Node>> {
        self.exprs(db).iter().fold(None, |acc, expr| {
            debug!("Checking if {:#?} overlaps with {:?}\n", expr, pos);
            if expr.overlap(pos) {
                debug!("{:?} overlaps with {:#?}\n", expr, pos);
                match acc {
                    Some(acc) => {
                        debug!(
                            "Comparing {:#?} with {:#?}\n",
                            expr.span_size(),
                            acc.span_size()
                        );
                        if expr.span_size() < acc.span_size() {
                            Some(expr.clone())
                        } else {
                            Some(acc)
                        }
                    }
                    None => Some(expr.clone()),
                }
            } else {
                acc
            }
        })
    }
}

#[salsa::tracked]
pub fn get_symbol(
    _db: &dyn Db,
    _src: text::SourceProgram,
    _pos: span::ByteOrLineColOrCoordInterned,
) -> Option<Spanned<ast::Node>> {
    None
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
    use span::{ByteOrLineColOrCoord, ByteOrLineColOrCoordInterned};

    use super::db::Database;
    use super::*;
    use insta::assert_snapshot;

    // test_span_text!(test_get_symbol, "fn main()[] {}", "main()", 0,0)
    // which expands to:
    // ```
    // #[okstd::log(debug)]
    // #[okstd::test]
    // fn test_get_symbol() {
    //     let db = &Database::default();
    //
    //     let src = text::SourceProgram::new(
    //         db,
    //         "inmemory://test".to_string(),
    //         r#"fn main()[] {}"#.to_string(),
    //     );
    //     let symb: Option<Spanned<Expression>> = get_symbol(
    //         db,
    //         src,
    //         ByteOrLineColOrCoordInterned::new(db, ByteOrLineColOrCoord::LineCol(0, 0)),
    //     );
    //   assert_snapshot!(symb.unwrap(), @r###"main()"###);
    // }```
    macro_rules! test_span_text {
        ($name:ident, $src:expr, $expected:expr, $line:expr, $col:expr) => {
            #[okstd::log(debug)]
            #[okstd::test]
            fn $name() {
                let db = &Database::default();

                let src = text::SourceProgram::new(
                    db,
                    "inmemory://test".to_string(),
                    $src.to_string(),
                );
                let symb: Option<Spanned<Node>> = get_symbol(
                    db,
                    src,
                    ByteOrLineColOrCoordInterned::new(db, ByteOrLineColOrCoord::LineCol($line, $col)),
                );
                if symb.is_none() {
                    panic!("Symbol not found");
                }
                assert_snapshot!(symb.unwrap(), @$expected);
            }
        };
    }

//     test_span_text!(
//         test_get_body,
//         r#"fn main()[] {
// let a = 1
//     }"#,
//         "let",
//         1,
//         1
//     );

//     test_span_text!(test_get_symbol, "fn main()[] {}", "priv fn", 0, 0);
}

#[salsa::tracked]
pub fn span_text(db: &dyn Db, span: text::Spanned) -> String {
    span.src(db).text(db)[span.span(db).span(db)].to_string()
}
